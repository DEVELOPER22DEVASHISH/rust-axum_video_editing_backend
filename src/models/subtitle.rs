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
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Video,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Video => Entity::belongs_to(video::Entity)
                .from(Column::VideoId)
                .to(video::Column::Id)
                .into(),
        }
    }
}

impl Related<video::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
