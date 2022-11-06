use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use response::RootResponse;

#[http_component]
fn get_root(_: Request) -> Result<Response> {
    let response_body = serde_json::to_string(&RootResponse::new())?;
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(response_body.into()))?)
}
