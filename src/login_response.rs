use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub plain_otp: String,
    pub user_id: String,
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
