use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QueryFilter, ColumnTrait,
    ActiveModelTrait,
};

use crate::models::admin;

pub struct AdminRepository;

impl AdminRepository {
    /// 按 ID 查询管理员
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<admin::Model>, DbErr> {
        admin::Entity::find_by_id(id).one(db).await
    }

    /// 按用户名查询管理员
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<admin::Model>, DbErr> {
        admin::Entity::find()
            .filter(admin::Column::Username.eq(username))
            .one(db)
            .await
    }

    /// 更新最后登录时间
    pub async fn update_last_login(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<admin::Model, DbErr> {
        let admin = admin::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("Admin with id {} not found", id)))?;

        let mut active_model = admin.into_active_model();
        active_model.last_login_at = sea_orm::ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        active_model.update(db).await
    }
}
