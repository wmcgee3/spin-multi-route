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
