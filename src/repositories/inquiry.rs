use sea_orm::*;

use crate::models::inquiry;

pub struct InquiryRepository;

impl InquiryRepository {
    /// 分页查询询盘
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<inquiry::Model>, u32), DbErr> {
        let query = inquiry::Entity::find();

        let total = query.clone().count(db).await?;

        let offset = ((page - 1) * limit) as u64;
        let items = query
            .order_by(inquiry::Column::Id, Order::Desc)
            .offset(Some(offset))
            .limit(Some(limit as u64))
            .all(db)
            .await?;

        Ok((items, total as u32))
    }

    /// 创建询盘
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        email: Option<String>,
        phone: Option<String>,
        message: String,
        product_id: Option<i32>,
        product_name: Option<String>,
    ) -> Result<inquiry::Model, DbErr> {
        let model = inquiry::ActiveModel {
            name: Set(name),
            email: Set(email),
            phone: Set(phone),
            message: Set(message),
            product_id: Set(product_id),
            product_name: Set(product_name),
            created_at: Set(crate::utils::time::now()),
            updated_at: Set(crate::utils::time::now()),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 按 ID 查询询盘
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<inquiry::Model>, DbErr> {
        inquiry::Entity::find_by_id(id).one(db).await
    }

    /// 删除询盘
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        inquiry::Entity::delete_by_id(id).exec(db).await
    }

    /// 批量删除询盘
    pub async fn delete_batch(db: &DatabaseConnection, ids: &[i32]) -> Result<DeleteResult, DbErr> {
        inquiry::Entity::delete_many()
            .filter(inquiry::Column::Id.is_in(ids.to_vec()))
            .exec(db)
            .await
    }
}
