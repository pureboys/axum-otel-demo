use sea_orm::{ConnectionTrait, ConnectOptions, Database, DatabaseConnection, DbErr, Schema};

use crate::models::user;

/// 初始化数据库连接并创建表
pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_string());
    opt.sqlx_logging(false);

    let db = Database::connect(opt).await?;

    // 使用 Entity 自动建表（if not exists）
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    let create_stmt = schema
        .create_table_from_entity(user::Entity)
        .if_not_exists()
        .to_owned();
    db.execute(&create_stmt).await?;

    Ok(db)
}
