use sea_orm::*;

use crate::models::user;

pub struct UserRepository;

impl UserRepository {
    /// 查询全部用户
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(db).await
    }

    /// 按 ID 查询用户
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    /// 按 username 查询用户
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
    }

    /// 创建用户
    pub async fn create(
        db: &DatabaseConnection,
        username: String,
        email: String,
    ) -> Result<user::Model, DbErr> {
        let model = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 更新用户
    pub async fn update(
        db: &DatabaseConnection,
        mut model: user::ActiveModel,
        username: Option<String>,
        email: Option<String>,
    ) -> Result<user::Model, DbErr> {
        if let Some(username) = username {
            model.username = Set(username);
        }
        if let Some(email) = email {
            model.email = Set(email);
        }
        model.update(db).await
    }

    /// 删除用户
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        user::Entity::delete_by_id(id).exec(db).await
    }
}
