use std::sync::Arc;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, web, HttpResponse, Result,
};
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::{debug, warn};
use url::Url;

use crate::shared_context::SharedContext;

#[derive(Debug, Deserialize)]
pub struct GetVodQuery {
    vod_url: String,
}

#[get("/get_vod")]
pub async fn get_vod_handler(
    shared_context: web::Data<Arc<SharedContext>>,
    query: web::Query<GetVodQuery>,
) -> Result<HttpResponse> {
    debug!("Processing /get_vod for {}", query.vod_url);

    let Ok(url) = Url::parse(&query.vod_url) else {
        warn!("Invalid url for /get_vod: {}", query.vod_url);
        return Err(ErrorBadRequest("Invalid URL"));
    };
    let kraken_response =
        shared_context
            .vod_storage
            .fetch(url)
            .await
            .map_err(|e: anyhow::Error| {
                let error_string = e.to_string();
                warn!(
                    "Error while fetching {} from kraken: {}",
                    query.vod_url, error_string
                );
                ErrorInternalServerError(error_string)
            })?;

    debug!("Successfully handled {}", query.vod_url);
    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .insert_header(("Content-Type", "binary/octet-stream"))
        .body(kraken_response))
}
