use tracing_subscriber::{EnvFilter, fmt};

/// Journaux lisibles en dev, JSON si `LOG_FORMAT=json` (production).
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,app=debug,tower_http=info"));
    let builder = fmt().with_env_filter(filter);
    if std::env::var("LOG_FORMAT").is_ok_and(|v| v == "json") {
        builder.json().init();
    } else {
        builder.init();
    }
}
