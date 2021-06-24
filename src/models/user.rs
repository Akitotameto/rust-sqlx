use warp::{Rejection, Reply};
use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPool;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserData {
    name: String,
}

impl User {
    // DBに接続
    pub async fn read() -> anyhow::Result<Vec<User>> {
        let pool = MySqlPool::connect("mysql://root:secret1012@127.0.0.1/sqlx_db").await?;
        let users = sqlx::query_as!(User,
            "
              SELECT id, name
              FROM users
            "
        )
        .fetch_all(&pool)
        .await?;

        Ok(users)
    }

    // pub fn create(body: UserData) -> Result<impl Reply, Rejection> {
    //     let connection = establish_connection();
    //     diesel::insert_into(users_schema::dsl::users)
    //     .values(body)
    //     .execute(&connection)
    //     .expect("Error saving new user");

    //     return Ok(warp::http::StatusCode::OK);
    // }

    // pub fn update(id: u64, body: UserData) -> Result<impl Reply, Rejection> {
    //     let connection = establish_connection();
    //     diesel::update(users_schema::dsl::users.find(id))
    //     .set(users_schema::name.eq(body.name))
    //     .execute(&connection)
    //     .expect("Error saving new user");

    //     return Ok(warp::http::StatusCode::OK);
    // }

    // pub fn delete(id: u64) -> Result<impl Reply, Rejection> {
    //     let connection = establish_connection();
    //     diesel::delete(users_schema::dsl::users.find(id))
    //     .execute(&connection)
    //     .expect("Error saving new user");

    //     return Ok(warp::http::StatusCode::OK);
    // }
}
