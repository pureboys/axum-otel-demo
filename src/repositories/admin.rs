use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, DeleteResult,
    EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::models::admin;

pub struct AdminRepository;

impl AdminRepository {
    /// 查询全部管理员
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<admin::Model>, DbErr> {
        admin::Entity::find().all(db).await
    }

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

    /// 创建管理员
    pub async fn create(
        db: &DatabaseConnection,
        username: String,
        password_hash: String,
        nickname: Option<String>,
        role: String,
    ) -> Result<admin::Model, DbErr> {
        let model = admin::ActiveModel {
            username: Set(username),
            password_hash: Set(password_hash),
            nickname: Set(nickname),
            role: Set(role),
            status: Set(1),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 更新管理员
    pub async fn update(
        db: &DatabaseConnection,
        mut model: admin::ActiveModel,
        nickname: Option<String>,
        role: Option<String>,
        status: Option<i8>,
    ) -> Result<admin::Model, DbErr> {
        if let Some(v) = nickname {
            model.nickname = Set(Some(v));
        }
        if let Some(v) = role {
            model.role = Set(v);
        }
        if let Some(v) = status {
            model.status = Set(v);
        }
        model.update(db).await
    }

    /// 删除管理员
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        admin::Entity::delete_by_id(id).exec(db).await
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
        active_model.last_login_at = Set(Some(crate::utils::time::now()));
        active_model.update(db).await
    }
}
