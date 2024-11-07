use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(ProjectThumbnails::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-project_thumbnails-refs-pk")
                            .table(ProjectThumbnails::Table)
                            .col(ProjectThumbnails::ProjectId)
                            .col(ProjectThumbnails::DataId)
                            ,
                    )
                    .col(integer(ProjectThumbnails::ProjectId))
                    .col(integer(ProjectThumbnails::DataId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project_thumbnails-projects")
                            .from(ProjectThumbnails::Table, ProjectThumbnails::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project_thumbnails-data")
                            .from(ProjectThumbnails::Table, ProjectThumbnails::DataId)
                            .to(Data::Table, Data::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectThumbnails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProjectThumbnails {
    Table,
    ProjectId,
    DataId,
    
}


#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Data {
    Table,
    Id,
}
