use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AllowList::Table)
                    .if_not_exists()
                    .col(pk_auto(AllowList::Id))
                    .col(big_unsigned(AllowList::UserSnowflake))
                    .col(big_unsigned(AllowList::GuildSnowflake))
                    .col(timestamp_null(AllowList::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(AllowList::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AllowList::Table, AllowList::UserSnowflake)
                            .to(User::Table, User::Snowflake),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AllowList::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AllowList {
    Table,
    Id,
    UserSnowflake,
    GuildSnowflake,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Snowflake,
    Banned,
    BanReason,
    SuperUser,
    CreatedAt,
    UpdatedAt,
}