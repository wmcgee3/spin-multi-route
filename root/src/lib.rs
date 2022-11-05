use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use utils::Link;

#[derive(Serialize)]
struct RootResponse {
    _links: Vec<Link>,
}

impl RootResponse {
    fn new() -> Self {
        RootResponse {
            _links: vec![
                Link::new("self", "/"),
                Link::new("hello", "/hello"),
                Link::new("things", "/things"),
            ],
        }
    }
}

#[http_component]
fn hello_spin(_: Request) -> Result<Response> {
    let response_body = serde_json::to_string(&RootResponse::new())?;
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(response_body.into()))?)
}
