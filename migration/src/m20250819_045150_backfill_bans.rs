use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let file = File::open("./banned_users.txt").map_err(|e| DbErr::Custom(e.to_string()))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| DbErr::Custom(e.to_string()))?;
            let user_id: u64 = match line.trim().parse() {
                Ok(id) => id,
                Err(_) => continue,
            };

            let stmt = Statement::from_sql_and_values(
                manager.get_database_backend(),
                        r#"
                            INSERT INTO "user" (snowflake, banned, ban_reason)
                            VALUES ($1, TRUE, $2)
                            ON CONFLICT (snowflake) DO UPDATE
                            SET banned = TRUE,
                                ban_reason = EXCLUDED.ban_reason
                            "#,
                [user_id.into(), "Legacy Ban".into()],
            );
            db.execute(stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
                .get_connection()
                .execute_unprepared("TRUNCATE TABLE \"user\"")
                .await?;

        Ok(())
    }
}
