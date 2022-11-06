use serde::Serialize;

use utils::get_things;

#[derive(Serialize)]
pub struct HelloResponse {
    pub _links: Vec<Link>,
    pub message: String,
}

impl HelloResponse {
    pub fn new(message: &str) -> Self {
        HelloResponse {
            _links: vec![Link::new("self", "/hello")],
            message: message.to_string(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Link {
    pub rel: String,
    pub href: String,
}

impl Link {
    pub fn new(rel: &str, href: &str) -> Self {
        Link {
            rel: rel.to_string(),
            href: href.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct RootResponse {
    pub _links: Vec<Link>,
}

impl RootResponse {
    pub fn new() -> Self {
        RootResponse {
            _links: vec![
                Link::new("self", "/"),
                Link::new("hello", "/hello"),
                Link::new("things", "/things"),
            ],
        }
    }
}

#[derive(Clone, Serialize)]
pub struct ThingResponse {
    pub _links: Vec<Link>,
    pub name: String,
    pub color: String,
}

impl ThingResponse {
    pub fn new(name: &str, color: &str) -> Self {
        ThingResponse {
            _links: vec![Link::new("self", &format!("/things/{}", name))],
            name: name.to_string(),
            color: color.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ThingsResponse {
    pub _links: Vec<Link>,
    pub items: Vec<ThingResponse>,
}

impl ThingsResponse {
    pub fn new() -> Self {
        let mut items = get_things()
            .iter()
            .fold(Vec::new(), |mut acc, (name, color)| {
                acc.push(ThingResponse::new(name, color));
                acc
            });
        items.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        ThingsResponse {
            _links: vec![Link::new("self", "/things")],
            items,
        }
    }
}
