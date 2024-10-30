use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Projects::Table)
                    .col(pk_auto(Projects::Id))
                    .col(string(Projects::Name))
                    .col(string(Projects::Technology))
                    .col(string(Projects::ShortDescription))
                    .col(string_null(Projects::Description))
                    .col(string(Projects::Category))
                    .col(string_null(Projects::GithubUrl))
                    .col(string_null(Projects::DownloadUrl))
                    .col(string_null(Projects::VisitUrl))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
    Name,
    Technology,
    ShortDescription,
    Description,
    Category,
    GithubUrl,
    DownloadUrl,
    VisitUrl,
    
}


