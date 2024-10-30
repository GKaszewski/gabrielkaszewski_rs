use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Jobs::Table)
                    .col(pk_auto(Jobs::Id))
                    .col(string(Jobs::Position))
                    .col(string(Jobs::Company))
                    .col(date(Jobs::StartDate))
                    .col(date_null(Jobs::EndDate))
                    .col(string(Jobs::Technologies))
                    .col(boolean(Jobs::StillWorking))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Jobs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Jobs {
    Table,
    Id,
    Position,
    Company,
    StartDate,
    EndDate,
    Technologies,
    StillWorking,
    
}


