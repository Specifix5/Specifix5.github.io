use std::collections::HashMap;
use std::error::Error;

use rocket::request::{ FromRequest, Outcome, Request };
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::tokio;
use rocket_governor::{ Method, Quota, RocketGovernable, RocketGovernor };
use crate::routes::ip::{ get_geoip_info, get_whois_info };
use crate::utils::constants::MAILBOX_MAX_LENGTH;
use crate::{
  ip_address,
  routes::ip::ENV_VARS,
  utils::{ constants::MAILBOX_RATE_LIMIT, logging::send_to_webhook, types::MailboxRequest },
};

pub struct ClientIp(String);

pub struct RateLimitGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientIp {
  type Error = ();

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let ip: Option<String> = ip_address!(request);

    match ip {
      Some(ip) => Outcome::Success(ClientIp(ip)),
      None => Outcome::Forward(Status::BadRequest),
    }
  }
}

impl<'r> RocketGovernable<'r> for RateLimitGuard {
  fn quota(_method: Method, _route_name: &str) -> Quota {
    Quota::per_second(Self::nonzero(MAILBOX_RATE_LIMIT.clone()))
  }
}

#[post("/mailbox", format = "application/json", data = "<body>")]
pub async fn mailbox(
  body: Json<MailboxRequest>,
  client_ip: ClientIp,
  _limitguard: RocketGovernor<'_, RateLimitGuard>
) -> status::Custom<String> {
  let ip_address: String = client_ip.0.clone();
  if body.content.is_empty() {
    return status::Custom(Status::BadRequest, "Message cannot be empty".to_string());
  } else if body.content.len() > MAILBOX_MAX_LENGTH.clone() {
    return status::Custom(Status::BadRequest, "Message too long!".to_string());
  }

  let _ = tokio::spawn(async move {
    let geoip_info: Result<HashMap<&str, String>, Box<dyn Error + Send + Sync>> = get_geoip_info(
      &ip_address
    ).await.map_err(|e: Box<dyn Error + Send + Sync>| e as Box<dyn Error + Send + Sync>);

    let whois_info: Result<HashMap<&str, String>, Box<dyn Error + Send + Sync>> = get_whois_info(
      &ip_address
    ).await.map_err(|e: Box<dyn Error + Send + Sync>| e as Box<dyn Error + Send + Sync>);

    if let Err(e) = &whois_info {
      eprintln!("Failed to fetch WHOIS info: {}", e);
    }

    if let Err(e) = &geoip_info {
      eprintln!("Failed to fetch GeoIP info: {}", e);
    }

    let city: &String = &geoip_info.as_ref().unwrap().get("City").unwrap().to_string();
    let subdivision: &String = &geoip_info.as_ref().unwrap().get("State").unwrap().to_string();
    let country: &String = &geoip_info.as_ref().unwrap().get("Country").unwrap().to_string();
    let flag: String = if country.contains("Unknown") {
      ":flag_aq:".to_string()
    } else {
      format!(":flag_{}:", country.to_lowercase())
    };

    let whois_data: &HashMap<&str, String> = &whois_info.as_ref().unwrap();

    send_to_webhook(
      format!(
        "**{} ({}, {}, {}{}) sent:** {} {}",
        ip_address,
        city,
        subdivision,
        country,
        flag,
        body.content,
        if
          ENV_VARS.read()
            .unwrap()
            .get("WHOIS")
            .map_or("false", |v| v) == "true"
        {
          format!(
            "\n```yaml\n{}\n```",
            whois_data
              .iter()
              .map(|(key, value)| format!("{}: {}", key, value))
              .collect::<Vec<String>>()
              .join("\n")
          ).to_string()
        } else {
          " ".to_string()
        }
      ).as_str(),
      "MAILBOX"
    ).await;
  });
  return status::Custom(Status::Ok, "Message sent.".to_string());
}
