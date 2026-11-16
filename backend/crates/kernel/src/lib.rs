//! Noyau partagé : identifiants, erreurs du domaine et pagination.
//! Aucun module métier ne dépend d'un autre ; tous dépendent du kernel.

mod error;
pub mod id;
mod id_conversions;
mod pagination;
mod validation;

pub use error::DomainError;
pub use pagination::{Page, PageRequest};
pub use validation::FieldError;

#[doc(hidden)]
pub use uuid;
