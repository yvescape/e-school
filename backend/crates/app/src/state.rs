use sqlx::PgPool;

/// État partagé par tous les handlers HTTP.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub const fn new(db: PgPool) -> Self {
        Self { db }
    }
}
