use sea_orm_migration::{prelude::*}; //, schema::*

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subtitle::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subtitle::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Subtitle::VideoId)
                            .integer()
                            .not_null()
                          
                    )
                    .col(ColumnDef::new(Subtitle::Text).string().not_null())
                    .col(ColumnDef::new(Subtitle::StartTime)
                        .double()
                        .not_null())
                    .col(ColumnDef::new(Subtitle::EndTime)
                        .double()
                        .not_null())
                    .col(ColumnDef::new(Subtitle::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),)

                      .foreign_key(
                                ForeignKey::create()
                                    .name("fk-subtitle-video-id")
                                    .from(Subtitle::Table, Subtitle::VideoId)
                                    .to(Video::Table, Video::Id)
                                    .on_delete(ForeignKeyAction::Cascade),
                            )    
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Subtitle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subtitle {
    Table,
    Id,
    VideoId,
    Text,
    StartTime,
    EndTime,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Video {
    Table,
    Id,
}