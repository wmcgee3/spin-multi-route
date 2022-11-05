use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use utils::{get_things, Link};

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
fn hello_spin(req: Request) -> Result<Response> {
    let item_name = req.uri().path().strip_prefix("/things/").unwrap();
    match get_things().get(item_name) {
        Some(color) => {
            let response_body = serde_json::to_string(&Thing::new(item_name, color))?;
            Ok(http::Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Some(response_body.into()))?)
        }
        None => Ok(http::Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(Some(r#"{"detail":"thing not found"}"#.into()))?),
    }
}
