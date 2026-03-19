use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::user::UserRepository;
use super::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};

pub struct UserService;

impl UserService {
    /// 获取所有用户
    pub async fn list_users(state: &AppState) -> Result<Vec<UserResponse>, AppError> {
        let users = UserRepository::find_all(&state.db)
            .await
            .map_err(AppError::from)?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// 获取用户详情
    pub async fn get_user(state: &AppState, id: i32) -> Result<UserResponse, AppError> {
        let user = UserRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("用户不存在".to_string()))?;
        Ok(UserResponse::from(user))
    }

    /// 创建用户
    pub async fn create_user(
        state: &AppState,
        req: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let user = UserRepository::create(&state.db, req.username, req.email)
            .await
            .map_err(AppError::from)?;
        Ok(UserResponse::from(user))
    }

    /// 更新用户
    pub async fn update_user(
        state: &AppState,
        id: i32,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let existing = UserRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("用户不存在".to_string()))?;

        let active_model = existing.into();
        let user = UserRepository::update(&state.db, active_model, req.username, req.email)
            .await
            .map_err(AppError::from)?;
        Ok(UserResponse::from(user))
    }

    /// 删除用户
    pub async fn delete_user(state: &AppState, id: i32) -> Result<(), AppError> {
        UserRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
