use app::{http::build_router, state::AppState};
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// Router dont la base est injoignable : suffit pour tout ce qui n'y touche pas.
pub fn offline_router(static_dir: &std::path::Path) -> Router {
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(300))
        .connect_lazy("postgres://nobody:none@127.0.0.1:1/none")
        .expect("lazy pool");
    build_router(AppState::new(pool), static_dir)
}

pub fn static_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "<html>e_school</html>").unwrap();
    dir
}
