use actix_cors::Cors;

use actix_web::middleware::Logger;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

pub fn setup_cors() -> Cors {
    Cors::default().allow_any_method().allow_any_origin()
}

pub fn setup_log_tracer(enable_middleware_logger: bool, env_filter: String) {
    if enable_middleware_logger {
        LogTracer::init().expect("Unable to setup log tracer");

        let subscriber = Registry::default()
            .with(fmt::layer())
            .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter)));
        set_global_default(subscriber).expect("Failed to set subscriber");
    }
}

pub fn setup_logger(enable_middleware_logger: bool) -> Logger {
    let mut logger_mw =
        actix_web::middleware::Logger::new("Peer: [%a], Real: [%{r}a]: %r -> %s, took %D ms");
    if !enable_middleware_logger {
        logger_mw = logger_mw.exclude_regex(".*");
    }

    logger_mw
}
