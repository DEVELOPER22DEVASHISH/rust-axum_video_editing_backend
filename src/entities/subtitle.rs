use sea_orm::entity::prelude::*;
use super::video;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "subtitle")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub video_id: i32,
    pub text: String,
    pub start_time: f64,
    pub end_time: f64,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "video::Entity",
        from = "Column::VideoId",
        to = "video::Column::Id"
    )]
    Video,
}

impl ActiveModelBehavior for ActiveModel {}
