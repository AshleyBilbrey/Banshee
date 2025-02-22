use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(big_unsigned_uniq(User::Snowflake))
                    .col(boolean(User::Banned).default(false))
                    .col(string_null(User::BanReason))
                    .col(boolean(User::SuperUser).default(false))
                    .col(timestamp_null(User::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(User::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_snowflake")
                    .table(User::Table)
                    .col(User::Snowflake)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Report::Table)
                    .if_not_exists()
                    .col(pk_auto(Report::Id))
                    .col(string(Report::MessageBody))
                    .col(string(Report::DisplayName))
                    .col(big_unsigned(Report::AuthorSnowflake))
                    .col(big_unsigned(Report::ReporterSnowflake))
                    .col(small_unsigned(Report::Status).default(0))
                    .col(timestamp_null(Report::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Report::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Report::Table, Report::AuthorSnowflake)
                            .to(User::Table, User::Snowflake),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Report::Table, Report::ReporterSnowflake)
                            .to(User::Table, User::Snowflake),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Whitelist::Table)
                    .if_not_exists()
                    .col(pk_auto(Whitelist::Id))
                    .col(big_unsigned(Whitelist::ServerSnowflake))
                    .col(big_unsigned(Whitelist::UserSnowflake))
                    .col(timestamp_null(Whitelist::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Whitelist::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Whitelist::Table, Whitelist::UserSnowflake)
                            .to(User::Table, User::Snowflake),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_snowflake")
                    .table(User::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Report::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Whitelist::Table).to_owned())
            .await?;

        Ok(())
    }
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

#[derive(DeriveIden)]
enum Report {
    Table,
    Id,
    MessageBody,
    DisplayName,
    AuthorSnowflake,
    ReporterSnowflake,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Whitelist {
    Table,
    Id,
    ServerSnowflake,
    UserSnowflake,
    CreatedAt,
    UpdatedAt,
}
