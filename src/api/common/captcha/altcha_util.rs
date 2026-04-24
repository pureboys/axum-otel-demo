//! ALTCHA PoW：签发挑战与校验登录时提交的 `altcha` 字段

use std::time::{SystemTime, UNIX_EPOCH};

use altcha::{
    create_challenge, verify_solution, Challenge, CreateChallengeOptions, Payload,
    VerifySolutionOptions,
};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use rand::RngExt;

use crate::config::RAW_CONFIG;
use crate::error::AppError;

fn load_secrets() -> Result<(String, String), AppError> {
    let c = RAW_CONFIG
        .get()
        .ok_or_else(|| AppError::Internal("服务未正确初始化".into()))?;
    let hmac = c
        .get_string("altcha.hmac_secret")
        .map_err(|_| AppError::Internal("缺少 altcha.hmac_secret 配置".into()))?;
    let key = c
        .get_string("altcha.hmac_key_secret")
        .map_err(|_| AppError::Internal("缺少 altcha.hmac_key_secret 配置".into()))?;
    if hmac.is_empty() || key.is_empty() {
        return Err(AppError::Internal("ALTCHA 密钥不能为空".into()));
    }
    Ok((hmac, key))
}

/// 签发新挑战（与官方 `http_server` 示例一致，使用 HMAC 签名 + 确定性模式 + 过期时间）
pub fn create_signed_challenge() -> Result<Challenge, AppError> {
    let (hmac_secret, key_secret) = load_secrets()?;
    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .as_secs()
        + 600;

    let counter: u32 = rand::rng().random_range(5_000u32..=10_000u32);

    let options = CreateChallengeOptions {
        algorithm: "PBKDF2/SHA-256".to_string(),
        cost: 5_000,
        counter: Some(counter),
        expires_at: Some(expires_at),
        hmac_signature_secret: Some(hmac_secret.clone()),
        hmac_key_signature_secret: Some(key_secret.clone()),
        ..Default::default()
    };

    create_challenge(options).map_err(|e| AppError::Internal(format!("签发 ALTCHA 失败: {e}")))
}

/// 校验 `alt`：Base64 JSON `Payload`（`challenge` + `solution`）
pub fn verify_client_payload(alt: &str) -> Result<(), AppError> {
    if alt.trim().is_empty() {
        return Err(AppError::Validation("请完成人机验证".into()));
    }
    let bytes = B64
        .decode(alt)
        .map_err(|_| AppError::Validation("验证码数据无效（Base64）".into()))?;
    let payload: Payload = serde_json::from_slice(&bytes)
        .map_err(|_| AppError::Validation("验证码数据格式错误".into()))?;
    let (hmac, key) = load_secrets()?;
    let r = verify_solution(VerifySolutionOptions {
        hmac_key_signature_secret: Some(key),
        ..VerifySolutionOptions::new(
            &payload.challenge,
            &payload.solution,
            hmac.as_str(),
        )
    })
    .map_err(|e| AppError::Validation(format!("验证码校验失败: {e}")))?;

    if r.expired {
        return Err(AppError::Validation("验证已过期，请刷新后重试".into()));
    }
    if !r.verified {
        if r.invalid_signature == Some(true) {
            return Err(AppError::Validation("验证码签名校验未通过".into()));
        }
        return Err(AppError::Validation("人机验证未通过，请重试".into()));
    }
    Ok(())
}
