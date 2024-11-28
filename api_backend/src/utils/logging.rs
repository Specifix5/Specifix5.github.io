use rocket::fairing::{ Fairing, Info, Kind };
use rocket::{ tokio, Request, Response };
use chrono::Local;
use webhook::client::WebhookClient;
use tokio::sync::mpsc;
use tokio::time::{ interval, Duration };
use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::time::Interval;

use crate::ip_address;
use crate::routes::ip::ENV_VARS;
pub struct RequestLogger;

#[macro_export(local_inner_macros)]
macro_rules! logger {
  ($text:expr) => {
    ::std::println!("{} - {}", ::chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"), $text);
  };
}

struct WebhookMessage {
  content: String,
  channel: String,
}

struct WebhookQueue {
  sender: mpsc::Sender<WebhookMessage>,
}

#[rocket::async_trait]
impl Fairing for RequestLogger {
  fn info(&self) -> Info {
    Info {
      name: "Request Logger",
      kind: Kind::Request | Kind::Response,
    }
  }

  async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::data::Data<'_>) {
    let ip_addr: Option<String> = ip_address!(request);
    logger!(
      format!(
        "Request {} - {} {}",
        ip_addr.unwrap_or("Unknown".to_string()),
        request.method(),
        request.uri()
      )
    );
  }

  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    let ip_addr: Option<String> = ip_address!(request);
    logger!(
      format!(
        "Response {} - {} {} {}",
        ip_addr.unwrap_or("Unknown".to_string()),
        request.method(),
        request.uri(),
        response.status()
      )
    );
  }
}

impl WebhookQueue {
  fn new() -> Arc<Self> {
    let (sender, mut receiver) = mpsc::channel::<WebhookMessage>(100);
    let queue: Arc<WebhookQueue> = Arc::new(WebhookQueue { sender });
    let queue_clone: Arc<WebhookQueue> = queue.clone();

    tokio::spawn(async move {
      let mut interval: Interval = interval(Duration::from_secs(1));
      let mut batch: Vec<WebhookMessage> = Vec::new();

      loop {
        tokio::select! {
          _ = interval.tick() => {

            if !batch.is_empty() {
              Self::send_batch(&batch).await;
              batch.clear();
            }
          }
          Some(msg) = receiver.recv() => {
            batch.push(msg);
            if batch.len() >= 2 {
              Self::send_batch(&batch).await;
              batch.clear();
              interval.reset();
            }
          }
        }
      }
    });

    queue_clone
  }

  async fn send_batch(batch: &[WebhookMessage]) {
    for msg in batch {
      let webhook: WebhookClient = WebhookClient::new(
        &ENV_VARS.read().unwrap().get(format!("{}_URL", msg.channel).as_str()).unwrap()
      );
      let _ = webhook.send(|m|
        m
          .content(&msg.content)
          .username(format!("{} @ specifix api", msg.channel.to_lowercase()).as_str())
          .avatar_url(&ENV_VARS.read().unwrap().get("AVATAR_URL").unwrap().as_str())
      ).await;
    }
  }

  pub async fn send_to_webhook(&self, text: &str, channel: &str) {
    let content: String = format!("``{}`` | {}", Local::now().format("%Y-%m-%dT%H:%M:%S"), text);
    let message: WebhookMessage = WebhookMessage {
      content,
      channel: channel.to_string(),
    };
    let _ = self.sender.send(message).await;
  }
}

// Initialize the WebhookQueue once in your application
lazy_static! {
  static ref WEBHOOK_QUEUE: Arc<WebhookQueue> = WebhookQueue::new();
}

// Use the queue to send messages
pub async fn send_to_webhook(text: &str, channel: &str) {
  WEBHOOK_QUEUE.send_to_webhook(text, channel).await;
}
