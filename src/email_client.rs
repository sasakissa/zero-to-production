use actix_web::http;
use reqwest::Client;

use crate::domain::{SubscriberEmail, SubscriberName};

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> EmailClient {
        EmailClient {
            http_client: reqwest::Client::new(),
            base_url: base_url,
            sender,
        }
    }
    pub fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!();
    }
}
