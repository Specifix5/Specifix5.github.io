use std::collections::HashMap;
use rocket::serde::json::Json;
use rocket_governor::{ Method, Quota, RocketGovernable, RocketGovernor };
use crate::utils::{ constants::DEFAULT_RATE_LIMIT, types::Index };

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
  fn quota(_method: Method, _route_name: &str) -> Quota {
    Quota::per_second(Self::nonzero(DEFAULT_RATE_LIMIT.clone()))
  }
}

#[get("/")]
pub async fn index(_limitguard: RocketGovernor<'_, RateLimitGuard>) -> Json<Index<'static>> {
  let mut endpoints: HashMap<&'static str, [&'static str; 2]> = HashMap::new();
  endpoints.insert("/ip", ["GET", "Returns the request IP Address."]);
  endpoints.insert("/mailbox", ["POST", "Sends a message to my anonymous mailbox"]);
  Json(Index { discord_server: "https://discord.specifix.dev/", endpoints: endpoints })
}
