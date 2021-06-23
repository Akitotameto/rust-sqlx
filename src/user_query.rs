use sqlx::mysql::MySqlPool;

pub async fn get_user(pool: &MySqlPool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
      r#"
          SELECT id
          FROM users
          ORDER BY id
      "#
      )
      .fetch_all(pool)
      .await?;

      for rec in recs {
          println!(
              "- {}",
              rec.id
          );
      }

    Ok(())
}

pub async fn add_user(pool: &MySqlPool) -> anyhow::Result<()> {
    // let mut tx = pool.begin().await?;
    let id = "3";
    let name = "アキラ";
    sqlx::query!(
        r#"
            INSERT INTO users(id, name)
            VALUES ( ?, ? )
        "#,
        id, name
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(())
}

pub  async fn update_user(pool: &MySqlPool, id: &str, name: &str) -> anyhow::Result<()> {
  sqlx::query!(
      r#"
          UPDATE users SET name = ? WHERE id = ?
      "#,
      name, id
  )
  .execute(pool)
  .await?;

  Ok(())
}

pub async fn delete_user(pool: &MySqlPool, id: &str) -> anyhow::Result<()> {
  sqlx::query!(
      r#"
          DELETE FROM users WHERE id = ?
      "#,
      id
  )
  .execute(pool)
  .await?;

  Ok(())
}