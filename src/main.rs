use sqlx::mysql::MySqlPool;
use rust_sqlx::user_query;
use rust_sqlx::user_query_as;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // DBに接続
    let pool = MySqlPool::connect("mysql://root:secret1012@127.0.0.1/sqlx_db").await?;

    // user_query_as::get_user(&pool).await?;

    // user_query::get_user(&pool).await?;
    user_query::add_user(&pool).await?;
    // user_query::update_user(&pool, "2", "あきと").await?;
    // user_query::delete_user(&pool, "2").await?;

    Ok(())
}