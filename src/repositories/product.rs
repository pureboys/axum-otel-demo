use sea_orm::prelude::Decimal;
use sea_orm::*;

use crate::models::product;

pub struct ProductRepository;

impl ProductRepository {
    /// 查询全部产品
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<product::Model>, DbErr> {
        product::Entity::find().all(db).await
    }

    /// 按 ID 查询产品
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<product::Model>, DbErr> {
        product::Entity::find_by_id(id).one(db).await
    }

    /// 按分类ID查询产品
    pub async fn find_by_category(
        db: &DatabaseConnection,
        category_id: i32,
    ) -> Result<Vec<product::Model>, DbErr> {
        product::Entity::find()
            .filter(product::Column::CategoryId.eq(category_id))
            .all(db)
            .await
    }

    /// 按状态查询产品
    pub async fn find_by_status(
        db: &DatabaseConnection,
        status: i8,
    ) -> Result<Vec<product::Model>, DbErr> {
        product::Entity::find()
            .filter(product::Column::Status.eq(status))
            .all(db)
            .await
    }

    /// 创建产品
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        description: String,
        price: Decimal,
        stock: i32,
        category_id: i32,
        image_url: String,
        status: i8,
    ) -> Result<product::Model, DbErr> {
        let model = product::ActiveModel {
            name: Set(name),
            description: Set(description),
            price: Set(price),
            stock: Set(stock),
            category_id: Set(category_id),
            image_url: Set(image_url),
            status: Set(status),
            created_at: Set(crate::utils::time::now()),
            updated_at: Set(crate::utils::time::now()),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 更新产品
    pub async fn update(
        db: &DatabaseConnection,
        mut model: product::ActiveModel,
        name: Option<String>,
        description: Option<String>,
        price: Option<Decimal>,
        stock: Option<i32>,
        category_id: Option<i32>,
        image_url: Option<String>,
        status: Option<i8>,
    ) -> Result<product::Model, DbErr> {
        if let Some(name) = name {
            model.name = Set(name);
        }
        if let Some(description) = description {
            model.description = Set(description);
        }
        if let Some(price) = price {
            model.price = Set(price);
        }
        if let Some(stock) = stock {
            model.stock = Set(stock);
        }
        if let Some(category_id) = category_id {
            model.category_id = Set(category_id);
        }
        if let Some(image_url) = image_url {
            model.image_url = Set(image_url);
        }
        if let Some(status) = status {
            model.status = Set(status);
        }
        model.updated_at = Set(crate::utils::time::now());
        model.update(db).await
    }

    /// 删除产品
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        product::Entity::delete_by_id(id).exec(db).await
    }
}
