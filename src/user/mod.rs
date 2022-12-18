use crate::ctx::Context;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use self::model::User;

pub mod model;

pub(crate) async fn get_all_users(State(ctx): State<Context>) -> impl IntoResponse {
    let users: Vec<User> = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&ctx.pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(users))
}
