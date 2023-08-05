use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn connect_db() -> Result<PgPool, Error> {
    let db_url = dotenv::var("DATABASE_URL").expect("db_url has not set!");
    sqlx::postgres::PgPool::connect(&db_url).await
}
