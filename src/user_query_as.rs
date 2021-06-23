use crate::models::user::User;
use sqlx::mysql::MySqlPool;

pub async fn get_user(pool: &MySqlPool) -> anyhow::Result<()> {
  let users = sqlx::query_as!(User,
      "
        SELECT id, name
        FROM users
      "
  )
  .fetch_all(pool)
  .await?;

  for user in users {
    println!(
        "- {} {}",
        user.id,
        user.name
    );
  }
  Ok(())
}