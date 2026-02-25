use crate::app::AppState;
use crate::dto::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::error::AppError;
use crate::repositories::user::UserRepository;

pub struct UserService;

impl UserService {
    /// 获取全部用户
    pub async fn list_users(state: &AppState) -> Result<Vec<UserResponse>, AppError> {
        let users = UserRepository::find_all(&state.db).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// 按 ID 获取用户
    pub async fn get_user(state: &AppState, id: i32) -> Result<UserResponse, AppError> {
        let user = UserRepository::find_by_id(&state.db, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;
        Ok(UserResponse::from(user))
    }

    /// 创建用户
    pub async fn create_user(
        state: &AppState,
        req: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        // 校验
        if req.username.trim().is_empty() {
            return Err(AppError::Validation("Username cannot be empty".into()));
        }
        if req.email.trim().is_empty() {
            return Err(AppError::Validation("Email cannot be empty".into()));
        }

        // 检查用户名是否已存在
        if UserRepository::find_by_username(&state.db, &req.username)
            .await?
            .is_some()
        {
            return Err(AppError::Validation(format!(
                "Username '{}' already exists",
                req.username
            )));
        }

        let user = UserRepository::create(&state.db, req.username, req.email).await?;
        Ok(UserResponse::from(user))
    }

    /// 更新用户
    pub async fn update_user(
        state: &AppState,
        id: i32,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let existing = UserRepository::find_by_id(&state.db, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;

        // 如果要修改 username，检查新名称是否冲突
        if let Some(ref new_username) = req.username {
            if new_username.trim().is_empty() {
                return Err(AppError::Validation("Username cannot be empty".into()));
            }
            if new_username != &existing.username {
                if UserRepository::find_by_username(&state.db, new_username)
                    .await?
                    .is_some()
                {
                    return Err(AppError::Validation(format!(
                        "Username '{}' already exists",
                        new_username
                    )));
                }
            }
        }

        let active_model: crate::models::user::ActiveModel = existing.into();
        let user = UserRepository::update(&state.db, active_model, req.username, req.email).await?;
        Ok(UserResponse::from(user))
    }

    /// 删除用户
    pub async fn delete_user(state: &AppState, id: i32) -> Result<(), AppError> {
        let result = UserRepository::delete(&state.db, id).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }
        Ok(())
    }
}
