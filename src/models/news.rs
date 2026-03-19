use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "news")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image: String,
    pub category_id: i32,
    pub author: String,
    pub view_count: i32,
    pub status: i8,
    pub is_featured: i8,
    pub published_at: Option<DateTime>,
    /// SEO 标题
    pub meta_title: Option<String>,
    /// SEO 描述
    pub meta_description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl ActiveModelBehavior for ActiveModel {}
