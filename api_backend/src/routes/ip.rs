use std::collections::HashMap;
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;

use env_file_reader::read_file;
use rocket::request::{ FromRequest, Outcome, Request };
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio;
use rocket_governor::{ Method, Quota, RocketGovernable, RocketGovernor };

use crate::utils::constants::DEFAULT_RATE_LIMIT;
use crate::utils::logging::send_to_webhook;
use crate::utils::types::Ip;
use whois_rust::{ WhoIs, WhoIsLookupOptions };
use std::sync::RwLock;
use once_cell::sync::Lazy;
use geoip2::{ City, Reader };

pub struct ClientIp(String);
pub struct RateLimitGuard;

static WHOIS: Lazy<RwLock<WhoIs>> = Lazy::new(|| {
  let whois = WhoIs::from_path("./static/servers.json").expect("Failed to load servers.json");
  RwLock::new(whois)
});

static GEOIP_BUFFER: Lazy<Vec<u8>> = Lazy::new(|| {
  std::fs::read("./static/GeoLite2-City.mmdb").expect("Failed to load GeoLite2-City.mmdb")
});

static GEOIP: Lazy<RwLock<Reader<City>>> = Lazy::new(|| {
  RwLock::new(Reader::<City>::from_bytes(&GEOIP_BUFFER).expect("Failed to load GeoLite2-City.mmdb"))
});

pub static ENV_VARS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  let env_variables = read_file("./.env").expect("Failed to read .env file");
  RwLock::new(env_variables)
});

#[macro_export]
macro_rules! ip_address {
  ($request:expr) => {
    $request.headers()
      .get_one("X-Forwarded-For")
      .map(|ip: &str | ip.to_string())
      .or_else(|| $request.client_ip().map(|ip| ip.to_string()))
  };
}

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
    Quota::per_second(Self::nonzero(DEFAULT_RATE_LIMIT.clone()))
  }
}

pub async fn get_geoip_info(
  ip: &str
) -> Result<HashMap<&str, String>, Box<dyn Error + Send + Sync>> {
  let ipaddr = IpAddr::from_str(ip).unwrap();
  let mut response: HashMap<&str, String> = HashMap::new();
  if ip != "127.0.0.1" {
    let reader = GEOIP.read().unwrap();
    let geoip = reader.lookup(ipaddr).unwrap();

    let city = geoip.city.unwrap().names.unwrap();
    let subdivisions = geoip.subdivisions.unwrap();
    let country = geoip.country.unwrap();

    if city.get("en").is_some_and(|v| !v.is_empty()) {
      response.insert("City", city.get("en").unwrap().to_string());
    }

    if subdivisions.get(0).is_some_and(|v| !v.iso_code.unwrap().is_empty()) {
      response.insert("State", subdivisions.get(0).unwrap().iso_code.unwrap().to_string());
    }

    if country.iso_code.is_some_and(|v| !v.is_empty()) {
      response.insert("Country", country.iso_code.unwrap().to_string());
    }
  }

  if response.get("City").is_none() {
    response.insert("City", "Unknown".to_string());
  }

  if response.get("State").is_none() {
    response.insert("State", "Unknown".to_string());
  }

  if response.get("Country").is_none() {
    response.insert("Country", "Unknown".to_string());
  }

  Ok(response)
}

pub async fn get_whois_info(
  ip: &str
) -> Result<HashMap<&str, String>, Box<dyn Error + Send + Sync>> {
  let mut fields: HashMap<&str, String> = HashMap::new();

  if
    ENV_VARS.read()
      .unwrap()
      .get("WHOIS")
      .is_some_and(|v| v == "true")
  {
    let whois = WHOIS.read().unwrap();
    let result: String = whois.lookup(WhoIsLookupOptions::from_string(ip).unwrap()).unwrap();

    for line in result.lines() {
      if fields.get("ISP").is_none() && (line.contains("OrgName:") || line.contains("descr:")) {
        fields.insert("ISP", line.split(':').nth(1).unwrap_or("").trim().to_string());
      }
      if
        fields.get("Abuse Email").is_none() &&
        (line.contains("OrgAbuseEmail:") || line.contains("abuse-mailbox:"))
      {
        fields.insert("Abuse Email", line.split(':').nth(1).unwrap_or("").trim().to_string());
      }
      if
        fields.get("Net Name").is_none() &&
        (line.contains("NetName:") || line.contains("netname:"))
      {
        fields.insert("Net Name", line.split(':').nth(1).unwrap_or("").trim().to_string());
      }

      // Break early if all fields are found
      if fields.len() == 3 {
        break;
      }
    }
  }

  let mut response: HashMap<&str, String> = HashMap::new();

  if let Some(isp) = fields.get("ISP") {
    response.insert("ISP", isp.clone());
  } else {
    response.insert("ISP", "Unknown".to_string());
  }

  if let Some(abuse_email) = fields.get("Abuse Email") {
    response.insert("Abuse Email", abuse_email.clone());
  } else {
    response.insert("Abuse Email", "Unknown".to_string());
  }

  if let Some(net_name) = fields.get("Net Name") {
    response.insert("Net Name", net_name.clone());
  } else {
    response.insert("Net Name", "Unknown".to_string());
  }

  Ok(response)
}

#[get("/ip")]
pub async fn ip(client_ip: ClientIp, _limitguard: RocketGovernor<'_, RateLimitGuard>) -> Json<Ip> {
  let ip_address: String = client_ip.0.clone();
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
        "{} requested GET /ip **({}, {}, {}{})**{}",
        ip_address,
        city,
        subdivision,
        country,
        flag,
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
      "LOGGING"
    ).await;
  });

  Json(Ip {
    ip: client_ip.0,
  })
}
