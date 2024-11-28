use std::collections::HashMap;
use rocket::serde::{ Deserialize, Serialize };

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Index<'r> {
  pub discord_server: &'r str,
  pub endpoints: HashMap<&'r str, [&'r str; 2]>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ip {
  pub ip: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct APIError<'r> {
  pub status: u16,
  pub error: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MailboxRequest {
  pub content: String,
}
