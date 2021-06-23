use sqlx::mysql::MySqlPool;
// use anyhow::{Result};

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let pool = MySqlPool::connect("mysql://root:secret1012@127.0.0.1/sqlx_db").await?;
    // get_user(&pool).await?;
    // add_user(&pool).await?;
    // update_user(&pool, "1", "まいこ").await?;
    delete_user(&pool, "1").await?;

    Ok(())
}

async fn get_user(pool: &MySqlPool) -> anyhow::Result<()> {
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

async fn add_user(pool: &MySqlPool) -> anyhow::Result<()> {
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

async fn update_user(pool: &MySqlPool, id: &str, name: &str) -> anyhow::Result<()> {
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

async fn delete_user(pool: &MySqlPool, id: &str) -> anyhow::Result<()> {
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