use app::{config::Config, db, http::build_router, server, state::AppState, telemetry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    telemetry::init();
    let config = Config::from_env()?;
    let pool = db::connect(&config.database_url).await?;
    db::migrate(&pool).await?;
    let router = build_router(AppState::new(pool), &config.static_dir);
    server::serve(config.bind_addr, router).await?;
    Ok(())
}
