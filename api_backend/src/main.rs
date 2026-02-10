#[macro_use]
extern crate rocket;

use crate::{ utils::logging::RequestLogger};
use rocket::config::LogLevel;
use rocket::figment::Figment;
use rocket::Config;
use rocket::http::Method;
use rocket_cors::{ AllowedHeaders, AllowedOrigins };

mod routes {
  pub mod index;
  pub mod ip;
  pub mod mailbox;
  pub mod _static;
  pub mod proxy;
}

mod utils {
  pub mod catcher;
  pub mod logging;
  pub mod types;
  pub mod constants;
}

use routes::index::index;
use routes::ip::ip;
use routes::_static::favicon;
use utils::catcher::{ bad_request, default, internal_error, not_found, rate_limit };
use routes::mailbox::mailbox;
use routes::proxy::proxy;

#[launch]
fn rocket() -> _ {
  let cors = (rocket_cors::CorsOptions {
    allowed_origins: AllowedOrigins::all(),
    allowed_methods: vec![Method::Get, Method::Post, Method::Options]
      .into_iter()
      .map(From::from)
      .collect(),
    allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
    allow_credentials: true,
    ..Default::default()
  }).to_cors();

  let config: Figment = Config::figment()
    .merge(("port", 3000))
    .merge(("log_level", LogLevel::Critical));

  // GET Requests
  rocket
    ::custom(config)
    .mount("/", routes![favicon])
    .mount("/", routes![index, ip, mailbox, proxy])
    .register("/", catchers![internal_error, not_found, bad_request, rate_limit, default])
    .attach(RequestLogger)
    .attach(cors.unwrap())
}
