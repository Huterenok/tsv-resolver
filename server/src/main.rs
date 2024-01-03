use std::time::Duration;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use tracing::info;
use tracing_actix_web::TracingLogger;

mod cli;
mod config;
mod handlers;
mod shared_context;
mod twitch_kraken;
mod vod_storage;

use cli::Cli;
use config::middleware;
use handlers::{cloudfront_proxy_handler, get_vod_handler};
use shared_context::SharedContext;

#[actix_web::main]
async fn main() {
    let cl_args = Cli::parse();

    middleware::setup_log_tracer(cl_args.enable_middleware_logger, "info".into());

    let shared_context = SharedContext::new(
        Duration::from_secs(cl_args.vod_cache_lifetime_sec),
        cl_args.vod_max_capacity,
    );

    info!("Starting server on {}", cl_args.bind_to);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::setup_cors())
            .wrap(TracingLogger::default())
            .wrap(middleware::setup_logger(cl_args.enable_middleware_logger))
            .app_data(web::Data::new(shared_context.clone()))
            .service(get_vod_handler)
            .service(cloudfront_proxy_handler)
            .service(Files::new("/", "./static/dist").index_file("index.html"))
    })
    .bind(cl_args.bind_to.clone())
    .expect("Unable to bind server")
    .run()
    .await
    .expect("Failed to start web server")
}
