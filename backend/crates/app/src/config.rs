use std::{net::SocketAddr, path::PathBuf};

/// Configuration lue depuis l'environnement (ou un fichier `.env`).
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: SocketAddr,
    pub static_dir: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("variable d'environnement manquante : {0}")]
    Missing(&'static str),
    #[error("valeur invalide pour {0} : {1}")]
    Invalid(&'static str, String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let database_url = get("DATABASE_URL").ok_or(ConfigError::Missing("DATABASE_URL"))?;
        let bind = get("BIND_ADDR").unwrap_or_else(|| "0.0.0.0:8080".into());
        let bind_addr = bind
            .parse()
            .map_err(|_| ConfigError::Invalid("BIND_ADDR", bind))?;
        let static_dir = get("STATIC_DIR")
            .unwrap_or_else(|| "./public".into())
            .into();
        Ok(Self {
            database_url,
            bind_addr,
            static_dir,
        })
    }
}
