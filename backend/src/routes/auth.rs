use axum::{extract::{State, Json}, response::IntoResponse};
use deadpool_sqlite::Pool;
use serde::Serialize;
use crate::user;

#[derive(Serialize)]
pub struct RegisterStatus {
    status: &'static str,
}

pub async fn register(
    State(Pool): State<Pool>,
    Json(payload): Json<user::User>
) -> Json<RegisterStatus> {
    Json(RegisterStatus { status: "ok" })
}
