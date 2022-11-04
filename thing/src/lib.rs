use anyhow::Result;
use querystring;
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
    color: Option<String>,
}

impl Thing {
    fn new(name: &str, color: Option<String>) -> Self {
        Thing {
            _links: vec![Link::new("self", &format!("/things/{}", name))],
            name: name.to_string(),
            color: if let Some(c) = color { Some(c) } else { None },
        }
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn hello_spin(req: Request) -> Result<Response> {
    let item_name = req.uri().path().strip_prefix("/things/").unwrap();
    let things = get_things();
    let color = match things.get(item_name) {
        Some(color) => Some(color.to_string()),
        None => {
            let mut color = None;
            if let Some(q) = req.uri().query() {
                for (key, value) in querystring::querify(q).iter().rev() {
                    if *key == "color" {
                        color = Some((*value).to_string());
                    }
                }
            }
            color
        }
    };
    let response_body = Thing::new(item_name, color);
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(serde_json::to_string(&response_body)?.into()))?)
}
