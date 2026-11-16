//! Couche HTTP : routage, erreurs, documentation OpenAPI, fichiers du frontend.

mod error;
mod health;
pub mod openapi;
mod problem;
mod router;
mod spa;

pub use error::ApiError;
pub use problem::Problem;
pub use router::build_router;
