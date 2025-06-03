pub use sea_orm_migration::prelude::*;

mod m20250601_084450_create_video_table;
mod m20250601_084504_create_subtitle_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250601_084450_create_video_table::Migration),
            Box::new(m20250601_084504_create_subtitle_table::Migration),
        ]
    }
}
