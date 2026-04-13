use std::path::Path;
use std::time::Instant;

use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use mime_guess::{from_path, mime};
use tokio::fs;

pub async fn serve_file(path: &Path, start: Instant) -> (StatusCode, Response, Instant) {
    let content = match fs::read(path).await {
        Ok(c) => c,
        Err(_) => {
            let resp = (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
            return (StatusCode::INTERNAL_SERVER_ERROR, resp, start);
        }
    };

    let mime = from_path(path).first_or_octet_stream();

    let mut headers = HeaderMap::new();
    let content_type = if mime.type_() == mime::TEXT {
        format!("{}; charset=utf-8", mime)
    } else {
        mime.to_string()
    };
    headers.insert("Content-Type", content_type.parse().unwrap());

    let resp = (StatusCode::OK, headers, content).into_response();
    (StatusCode::OK, resp, start)
}
