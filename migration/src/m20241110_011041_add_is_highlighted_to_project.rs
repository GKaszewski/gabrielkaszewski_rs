use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Projects {
    Table,
    IsHighlighted,
    IsArchived,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //
        // add column
        //
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column_if_not_exists(boolean(Projects::IsHighlighted).default(false))
                    .add_column_if_not_exists(boolean(Projects::IsArchived).default(false))
                    .to_owned(),
            )
            .await

        //
        // delete column
        //
        /*
        manager
            .alter_table(
                Table::alter()
                    .table(Movies::Table)
                    .drop_column(Movies::Rating)
                    .to_owned(),
            )
            .await
        */

        //
        // create index
        //
        /*
        manager
            .create_index(
                Index::create()
                    .name("idx-movies-rating")
                    .table(Movies::Table)
                    .col(Movies::Rating)
                    .to_owned(),
            )
            .await;
        */
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Projects::IsHighlighted)
                    .drop_column(Projects::IsArchived)
                    .to_owned(),
            )
            .await
    }
}

