use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use utils::Link;

#[derive(Serialize)]
struct StuffResponse {
    _links: Vec<Link>,
    message: String,
}

impl StuffResponse {
    fn new(_links: &Vec<Link>, message: &str) -> Self {
        StuffResponse {
            _links: _links.to_vec(),
            message: message.to_string(),
        }
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn hello_spin(_: Request) -> Result<Response> {
    let response_body = StuffResponse::new(&vec![Link::new("self", "/stuff")], "Hello, Spin!");
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(serde_json::to_string(&response_body)?.into()))?)
}
