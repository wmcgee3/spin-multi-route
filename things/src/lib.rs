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
    fn new() -> Self {
        let mut items = get_things()
            .iter()
            .fold(Vec::new(), |mut acc, (name, color)| {
                acc.push(Thing::new(name, color));
                acc
            });
        items.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        ThingsResponse {
            _links: vec![Link::new("self", "/things")],
            items,
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

#[http_component]
fn hello_spin(_: Request) -> Result<Response> {
    let response_body = serde_json::to_string(&ThingsResponse::new())?;
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(response_body.into()))?)
}
