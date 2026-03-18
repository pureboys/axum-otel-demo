use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

use crate::app::AppState;
use crate::config::RAW_CONFIG;
use crate::error::AppError;
use crate::repositories::admin::AdminRepository;

use super::dto::{AdminInfo, LoginResponse, TokenResponse};

/// 获取 JWT 配置
fn get_jwt_config() -> (String, i64) {
    let config = RAW_CONFIG.get().expect("Config not initialized");
    let secret = config.get_string("jwt.secret").unwrap_or_else(|_| "default-secret".to_string());
    let expire_seconds = config.get_int("jwt.expire_seconds").unwrap_or(86400) as i64;
    (secret, expire_seconds)
}

/// JWT 载荷
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // 用户ID
    pub username: String,   // 用户名
    pub role: String,       // 角色
    pub exp: i64,           // 过期时间
    pub iat: i64,           // 签发时间
}

/// 认证服务
pub struct AuthService;

impl AuthService {
    /// 登录验证
    pub async fn login(
        state: &AppState,
        username: String,
        password: String,
    ) -> Result<LoginResponse, AppError> {
        // 查询管理员
        let admin = AdminRepository::find_by_username(&state.db, &username)
            .await?
            .ok_or_else(|| AppError::AuthFailed("用户名或密码错误".into()))?;

        // 检查状态
        if admin.status == 0 {
            return Err(AppError::AuthFailed("账号已被禁用".into()));
        }

        // 验证密码
        let password_hash = admin.password_hash.clone();
        let is_valid = bcrypt::verify(&password, &password_hash)
            .map_err(|_| AppError::AuthFailed("密码验证失败".into()))?;

        if !is_valid {
            return Err(AppError::AuthFailed("用户名或密码错误".into()));
        }

        // 更新最后登录时间
        let _ = AdminRepository::update_last_login(&state.db, admin.id).await;

        // 保存需要的信息
        let admin_id = admin.id;
        let admin_username = admin.username.clone();
        let admin_nickname = admin.nickname;
        let admin_role = admin.role.clone();

        // 获取 JWT 配置并生成 Token
        let (secret, expire_seconds) = get_jwt_config();
        let token = Self::generate_token(admin_id, admin_username.clone(), admin_role.clone(), &secret, expire_seconds)?;

        Ok(LoginResponse {
            token,
            token_type: "Bearer".into(),
            expires_in: expire_seconds,
            admin: AdminInfo {
                id: admin_id,
                username: admin_username,
                nickname: admin_nickname,
                role: admin_role,
            },
        })
    }

    /// 验证 Token
    pub fn verify_token(token: &str) -> Result<Claims, AppError> {
        let (secret, _) = get_jwt_config();
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map_err(|_| AppError::Unauthorized("无效的 Token".into()))?;

        Ok(token_data.claims)
    }

    /// 刷新 Token
    pub fn refresh_token(claims: &Claims) -> Result<TokenResponse, AppError> {
        let (secret, expire_seconds) = get_jwt_config();
        let token = Self::generate_token(
            claims.sub.parse().unwrap_or(0),
            claims.username.clone(),
            claims.role.clone(),
            &secret,
            expire_seconds,
        )?;

        Ok(TokenResponse {
            token,
            token_type: "Bearer".into(),
            expires_in: expire_seconds,
        })
    }

    /// 生成 Token
    fn generate_token(id: i32, username: String, role: String, secret: &str, expire_seconds: i64) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(expire_seconds);

        let claims = Claims {
            sub: id.to_string(),
            username,
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal("生成 Token 失败".into()))
    }
}
