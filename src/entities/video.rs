use sea_orm::entity::prelude::*;
use super::subtitle;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "video")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub original_name: String,
    pub file_path: String,
    pub duration: Option<f64>,
    pub size: i32,
    pub status: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "subtitle::Entity")]
    Subtitle,
}

impl ActiveModelBehavior for ActiveModel {}
