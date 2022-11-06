use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use responses::ThingResponse;
use utils::get_things;

#[http_component]
fn get_thing(req: Request) -> Result<Response> {
    let item_name = req.uri().path().strip_prefix("/things/").unwrap();
    let (status_code, response_body) = match get_things().get(item_name) {
        Some(color) => (
            200,
            serde_json::to_string(&ThingResponse::new(item_name, color))?,
        ),
        None => (404, r#"{"detail":"thing not found"}"#.to_string()),
    };
    Ok(http::Response::builder()
        .status(status_code)
        .header("content-type", "application/json")
        .body(Some(response_body.into()))?)
}
