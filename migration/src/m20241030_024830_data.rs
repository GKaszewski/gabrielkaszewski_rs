use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Data::Table)
                    .col(pk_auto(Data::Id))
                    .col(string(Data::FileUrl))
                    .col(boolean(Data::Protected))
                    .col(string(Data::FileName))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Data::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Data {
    Table,
    Id,
    FileUrl,
    Protected,
    FileName,
}
