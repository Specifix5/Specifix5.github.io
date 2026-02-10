use rocket::{
  http::Status,
  response::status,
  serde::json::Json,
  serde::json::Value,
};
use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};

use crate::routes::ip::ENV_VARS;

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
  fn quota(_method: Method, _route_name: &str) -> Quota {
    Quota::per_second(Self::nonzero(32))
  }
}

#[post("/proxy?<key>&<url>", format = "application/json", data = "<body>")]
pub async fn proxy(
  key: &str,
  url: &str,
  body: Json<Value>,
  _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> status::Custom<String> {
  if key != ENV_VARS.read().unwrap().get("PROXY_KEY").unwrap().as_str() {
    return status::Custom(Status::Unauthorized, "invalid key".into());
  }
    
  if !url.starts_with("https://") {
    return status::Custom(Status::BadRequest, "invalid url".into());
  }

  let client = reqwest::Client::new();

  let resp = match client
    .post(url)
    .header("Content-Type", "application/json")
    .json(&*body)
    .send()
    .await
  {
    Ok(r) => r,
    Err(e) => {
      return status::Custom(
        Status::BadGateway,
        format!("upstream request failed: {e}"),
      )
    }
  };

  let status_code = resp.status();
  let text = match resp.text().await {
    Ok(t) => t,
    Err(e) => {
      return status::Custom(
        Status::BadGateway,
        format!("failed reading upstream response: {e}"),
      )
    }
  };

  let rocket_status = Status::from_code(status_code.as_u16()).unwrap_or(Status::BadGateway);

  status::Custom(rocket_status, text)
}
