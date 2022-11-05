use std::collections::HashMap;

use serde::Serialize;

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

pub fn get_things() -> HashMap<String, String> {
    HashMap::from([
        ("apple".to_string(), "red".to_string()),
        ("banana".to_string(), "yellow".to_string()),
        ("pickle".to_string(), "green".to_string()),
    ])
}
