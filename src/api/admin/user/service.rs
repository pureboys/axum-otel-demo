use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::admin::AdminRepository;
use super::dto::{AdminUserResponse, CreateAdminRequest, UpdateAdminRequest};

pub struct AdminUserService;

impl AdminUserService {
    /// 获取所有管理员
    pub async fn list_admins(state: &AppState) -> Result<Vec<AdminUserResponse>, AppError> {
        let admins = AdminRepository::find_all(&state.db)
            .await
            .map_err(AppError::from)?;
        Ok(admins.into_iter().map(AdminUserResponse::from).collect())
    }

    /// 获取管理员详情
    pub async fn get_admin(state: &AppState, id: i32) -> Result<AdminUserResponse, AppError> {
        let admin = AdminRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("管理员不存在".to_string()))?;
        Ok(AdminUserResponse::from(admin))
    }

    /// 创建管理员
    pub async fn create_admin(
        state: &AppState,
        req: CreateAdminRequest,
    ) -> Result<AdminUserResponse, AppError> {
        let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
            .map_err(|_| AppError::Internal("密码加密失败".to_string()))?;
        let role = req.role.unwrap_or_else(|| "admin".to_string());
        let admin = AdminRepository::create(&state.db, req.username, password_hash, req.nickname, role)
            .await
            .map_err(AppError::from)?;
        Ok(AdminUserResponse::from(admin))
    }

    /// 更新管理员
    pub async fn update_admin(
        state: &AppState,
        id: i32,
        req: UpdateAdminRequest,
    ) -> Result<AdminUserResponse, AppError> {
        let existing = AdminRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("管理员不存在".to_string()))?;

        use sea_orm::IntoActiveModel;
        let active_model = existing.into_active_model();
        let admin = AdminRepository::update(&state.db, active_model, req.nickname, req.role, req.status)
            .await
            .map_err(AppError::from)?;
        Ok(AdminUserResponse::from(admin))
    }

    /// 修改管理员密码
    pub async fn change_password(
        state: &AppState,
        id: i32,
        new_password: &str,
    ) -> Result<(), AppError> {
        let existing = AdminRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("管理员不存在".to_string()))?;

        let password_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
            .map_err(|_| AppError::Internal("密码加密失败".to_string()))?;

        use sea_orm::{ActiveModelTrait, ActiveValue::Set, IntoActiveModel};
        let mut active_model = existing.into_active_model();
        active_model.password_hash = Set(password_hash);
        let _: crate::models::admin::Model = active_model.update(&state.db).await.map_err(AppError::from)?;
        Ok(())
    }

    /// 删除管理员
    pub async fn delete_admin(state: &AppState, id: i32) -> Result<(), AppError> {
        AdminRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
