use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::Request;

use crate::utils::constants::{
  MESSAGE_ERROR_400,
  MESSAGE_ERROR_404,
  MESSAGE_ERROR_429,
  MESSAGE_ERROR_500,
};
use crate::utils::types::APIError;

#[catch(500)]
pub fn internal_error() -> Json<APIError<'static>> {
  Json(APIError { error: &MESSAGE_ERROR_500, status: 500 })
}

#[catch(404)]
pub fn not_found() -> Json<APIError<'static>> {
  Json(APIError { error: &MESSAGE_ERROR_404, status: 404 })
}

#[catch(400)]
pub fn bad_request() -> Json<APIError<'static>> {
  Json(APIError { error: &MESSAGE_ERROR_400, status: 400 })
}

#[catch(429)]
pub fn rate_limit() -> Json<APIError<'static>> {
  Json(APIError { error: &MESSAGE_ERROR_429, status: 429 })
}

#[catch(default)]
pub fn default(status: Status, _: &Request) -> Json<APIError<'static>> {
  Json(APIError { error: &MESSAGE_ERROR_404, status: status.code })
}
