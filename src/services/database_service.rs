use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let password = env::var("POSTGRES_PASSWORD").expect("missing POSTGRES_PASSWORD");
    let connection_string = format!("postgres://postgres:{}@database:5432/postgres", password);
    let connection = Database::connect(connection_string).await?;

    Ok(connection)
}
