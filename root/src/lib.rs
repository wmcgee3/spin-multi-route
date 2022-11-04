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
    fn new(_links: &Vec<Link>) -> Self {
        RootResponse {
            _links: _links.to_vec(),
        }
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn hello_spin(_: Request) -> Result<Response> {
    let response_body = RootResponse::new(&vec![
        Link::new("self", "/"),
        Link::new("stuff", "/stuff"),
        Link::new("things", "/things"),
    ]);
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(serde_json::to_string(&response_body)?.into()))?)
}
