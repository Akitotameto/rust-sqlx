use rust_sqlx::routes::users_api;
use rust_sqlx::user_query;
// use rust_sqlx::user_query_as
// use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    // warp::serve(hello).run(([0, 0, 0, 0], 3000)).await;

    warp::serve(users_api())
        .run(([127, 0, 0, 1], 3000))
        .await;

    // user_query_as::get_user(&pool).await?;
    // user_query_as::add_user(&pool).await?;
    // user_query_as::update_user(&pool, "5", "田中太郎").await?;
    // user_query_as::delete_user(&pool, "5").await?;

    // user_query::get_user(&pool).await?;
    // user_query::add_user(&pool).await?;
    // user_query::update_user(&pool, "2", "あきと").await?;
    // user_query::delete_user(&pool, "2").await?;

    Ok(())
}