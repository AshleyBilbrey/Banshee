use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let connection_string = env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let connection = Database::connect(connection_string).await?;

    Ok(connection)
}
