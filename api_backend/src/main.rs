#[macro_use]
extern crate rocket;

use crate::utils::logging::RequestLogger;
use rocket::config::LogLevel;
use rocket::figment::Figment;
use rocket::Config;

mod routes {
  pub mod index;
  pub mod ip;
  pub mod mailbox;
  pub mod _static;
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
#[launch]
fn rocket() -> _ {
  let config: Figment = Config::figment()
    .merge(("port", 3000))
    .merge(("log_level", LogLevel::Critical));

  // GET Requests
  rocket
    ::custom(config)
    .mount("/", routes![favicon])
    .mount("/", routes![index, ip, mailbox])
    .register("/", catchers![internal_error, not_found, bad_request, rate_limit, default])
    .attach(RequestLogger)
}
