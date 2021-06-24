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

pub async fn add_user(pool: &MySqlPool) -> anyhow::Result<()> {
    let id = "5";
    let name = "山本太郎";
    sqlx::query_as!(User,
      "
        INSERT INTO users(id, name)
        VALUES ( ?, ? )
      ",
      id,name
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(())
}

pub  async fn update_user(pool: &MySqlPool, id: &str, name: &str) -> anyhow::Result<()> {
  sqlx::query_as!(User,
    "
      UPDATE users SET name = ? WHERE id = ?
    ",
    name, id
  )
  .execute(pool)
  .await?;

  Ok(())
}

pub async fn delete_user(pool: &MySqlPool, id: &str) -> anyhow::Result<()> {
  sqlx::query_as!(User,
    "
    DELETE FROM users WHERE id = ?
    ",
    id
  )
  .execute(pool)
  .await?;

  Ok(())
}