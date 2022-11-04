use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use utils::{get_things, Link};

#[derive(Serialize)]
struct ThingsResponse {
    _links: Vec<Link>,
    items: Vec<Thing>,
}

impl ThingsResponse {
    fn new(_links: &Vec<Link>, items: &Vec<Thing>) -> Self {
        ThingsResponse {
            _links: _links.to_vec(),
            items: items.to_vec(),
        }
    }
}

#[derive(Clone, Serialize)]
struct Thing {
    _links: Vec<Link>,
    name: String,
    color: String,
}

impl Thing {
    fn new(name: &str, color: &str) -> Self {
        Thing {
            _links: vec![Link::new("self", &format!("/things/{}", name))],
            name: name.to_string(),
            color: color.to_string(),
        }
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn hello_spin(_: Request) -> Result<Response> {
    let mut items = Vec::new();
    for (key, value) in get_things().iter() {
        items.push(Thing::new(key, value));
    }
    let response_body = ThingsResponse::new(&vec![Link::new("self", "/things")], &items);
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(serde_json::to_string(&response_body)?.into()))?)
}
