use sea_orm::DatabaseConnection;

pub async fn establish_connection() -> DatabaseConnection {
    let connection_string: &str = "username:password@host/database?currentSchema=my_schema";
    let db: DatabaseConnection =
        Database::connect("protocol://username:password@host/database").await?;
}
