use sea_orm_migration::{prelude::*}; // , schema::*

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Video::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Video::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key())
                    .col(ColumnDef::new(Video::OriginalName)
                        .string()
                        .not_null())
                    .col(ColumnDef::new(Video::FilePath)
                        .string()
                        .not_null())
                    .col(ColumnDef::new(Video::Duration)
                        .double())
                    .col(ColumnDef::new(Video::FileSize)
                        .big_integer()
                        .not_null())
                    .col(ColumnDef::new(Video::Status)
                        .string()
                        .not_null()
                        .default("uploaded"))
                    .col(ColumnDef::new(Video::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Video::UpdatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())                    
                        )
                    .to_owned(),
                    )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Video::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Video {
    Table,
    Id,
    OriginalName,
    FilePath,
    Duration,
    Status,
    FileSize,
    CreatedAt,
    UpdatedAt,
    
}
