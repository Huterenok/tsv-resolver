use actix_web::error::ErrorInternalServerError;
use actix_web::web::{self};
use actix_web::{get, HttpResponse, Result};
use reqwest::StatusCode;
use tracing::warn;

#[get("/{tail:.*\\.ts}")]
pub async fn cloudfront_proxy_handler(path: web::Path<String>) -> Result<HttpResponse> {
    let client = reqwest::ClientBuilder::new()
        .use_rustls_tls()
        .build()
        .expect("Failed to build reqwest client");
    let cloudfront_res = client
        .get(format!("https://{}", path))
        .send()
        .await
        .map_err(|e| {
            warn!("Error while fetching from cloudfront: {:?}", e);
            ErrorInternalServerError(e.to_string())
        })?;

    let body = cloudfront_res.bytes().await.map_err(|e| {
        warn!(
            "Error while transforming cloudfront response body to bytes: {:?}",
            e
        );
        ErrorInternalServerError(e.to_string())
    })?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .insert_header(("Content-Type", "binary/octet-stream"))
        .body(body))
}
