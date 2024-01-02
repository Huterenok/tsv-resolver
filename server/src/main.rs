use std::time::Duration;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod cli;
mod handlers;
mod shared_context;
mod twitch_kraken;
mod vod_storage;

use cli::Cli;
use handlers::{cloudfront_proxy_handler, get_vod_handler};
use shared_context::SharedContext;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cl_args = Cli::parse();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let shared_context = SharedContext::new(
        Duration::from_secs(cl_args.vod_cache_lifetime_sec),
        cl_args.vod_max_capacity,
    );

    info!("Starting server on {}", cl_args.bind_to);

    HttpServer::new(move || {
        let mut logger_mw =
            middleware::Logger::new("Peer: [%a], Real: [%{r}a]: %r -> %s, took %D ms");
        if !cl_args.enable_middleware_logger {
            logger_mw = logger_mw.exclude_regex(".*");
        }

        App::new()
            .wrap(logger_mw)
            .wrap(Cors::default().allow_any_method().allow_any_origin())
            .app_data(web::Data::new(shared_context.clone()))
            .service(get_vod_handler)
            .service(cloudfront_proxy_handler)
            .service(Files::new("/", "./static/dist").index_file("index.html"))
    })
    .bind(cl_args.bind_to)?
    .run()
    .await?;

    Ok(())
}
