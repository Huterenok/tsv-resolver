use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Address on which server will listen to HTTP requests
    #[arg(long, default_value = "0.0.0.0:8080")]
    pub bind_to: String,

    /// How long vod will be valid in cache (in seconds)
    #[arg(long, default_value_t = 600)]
    pub vod_cache_lifetime_sec: u64,

    /// Max capacity of the cache
    #[arg(long, default_value_t = 100)]
    pub vod_max_capacity: u64,

    /// Enable Middleware Logger (always on info log level)
    /// Middleware logger writes log on every request, use carefully
    #[arg(long, default_value_t = true)]
    pub enable_middleware_logger: bool,
}
