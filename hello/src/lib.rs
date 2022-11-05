use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use utils::Link;

#[derive(Serialize)]
struct HelloResponse {
    _links: Vec<Link>,
    message: String,
}

impl HelloResponse {
    fn new(message: &str) -> Self {
        HelloResponse {
            _links: vec![Link::new("self", "/hello")],
            message: message.to_string(),
        }
    }
}

#[http_component]
fn hello_spin(_: Request) -> Result<Response> {
    let response_body = serde_json::to_string(&HelloResponse::new("Hello from Spin!"))?;
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(response_body.into()))?)
}