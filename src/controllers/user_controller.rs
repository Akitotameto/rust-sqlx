use warp::{Rejection, Reply};
// use anyhow::{Context, Result};
use crate::models::user;

pub async fn get_users_handler() -> anyhow::Result<> {
  let users = user::User::read();

  // Err(anyhow::Error)
  // Ok(warp::reply::json(&users))
  // return Ok(warp::reply());
  Ok(())
}

// pub async fn post_user_handler(body: user::UserData) -> Result<impl Reply, Rejection> {
//   return user::User::create(body);
// }

// pub async fn put_user_handler(id:u64, body: user::UserData) -> Result<impl Reply, Rejection> {
//   return user::User::update(id, body);
// }

// pub async fn delete_user_handler(id:u64) -> Result<impl Reply, Rejection> {
//   return user::User::delete(id);
// }