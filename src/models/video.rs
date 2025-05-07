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
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Subtitle,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Subtitle => Entity::has_many(subtitle::Entity).into(),
        }
    }
}

impl Related<subtitle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subtitle.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}




