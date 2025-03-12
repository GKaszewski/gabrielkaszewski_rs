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
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column_if_not_exists(boolean(Projects::IsHighlighted).default(false))
                    .to_owned(),
            )
            .await?;

        // Add `is_archived` column
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column_if_not_exists(boolean(Projects::IsArchived).default(false))
                    .to_owned(),
            )
            .await?;

        Ok(())
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

