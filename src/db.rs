use sea_orm::{ConnectionTrait, ConnectOptions, Database, DatabaseConnection, DbErr, Schema};

use crate::models::{inquiry, user};

/// 初始化数据库连接并创建表
pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_string());
    opt.sqlx_logging(false);

    let db = Database::connect(opt).await?;

    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    let create_user = schema
        .create_table_from_entity(user::Entity)
        .if_not_exists()
        .to_owned();
    db.execute(&create_user).await?;

    let create_inquiry = schema
        .create_table_from_entity(inquiry::Entity)
        .if_not_exists()
        .to_owned();
    db.execute(&create_inquiry).await?;

    Ok(db)
}
