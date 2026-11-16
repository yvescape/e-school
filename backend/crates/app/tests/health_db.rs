//! Nécessite PostgreSQL : `docker compose -f ../deploy/compose.dev.yml up -d`
//! puis `DATABASE_URL` (voir backend/.env.example).
mod common;

use app::{http::build_router, state::AppState};
use common::{get, static_dir};

#[sqlx::test(migrations = "../../migrations")]
async fn readiness_succeeds_with_database(pool: sqlx::PgPool) {
    let dir = static_dir();
    let res = get(
        build_router(AppState::new(pool), dir.path()),
        "/api/health/ready",
    )
    .await;
    assert_eq!(res.status, 200);
    assert_eq!(res.json()["database"], "ok");
}
