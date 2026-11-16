use app::config::Config;
use std::collections::HashMap;

fn load(vars: &[(&str, &str)]) -> Result<Config, app::config::ConfigError> {
    let map: HashMap<String, String> = vars
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect();
    Config::from_lookup(|key| map.get(key).cloned())
}

#[test]
fn database_url_is_required() {
    assert!(load(&[]).is_err());
}

#[test]
fn applies_defaults() {
    let cfg = load(&[("DATABASE_URL", "postgres://x")]).unwrap();
    assert_eq!(cfg.database_url, "postgres://x");
    assert_eq!(cfg.bind_addr.to_string(), "0.0.0.0:8080");
    assert_eq!(cfg.static_dir.to_str(), Some("./public"));
}

#[test]
fn reads_overrides() {
    let cfg = load(&[
        ("DATABASE_URL", "postgres://x"),
        ("BIND_ADDR", "127.0.0.1:9000"),
        ("STATIC_DIR", "/srv/web"),
    ])
    .unwrap();
    assert_eq!(cfg.bind_addr.port(), 9000);
    assert_eq!(cfg.static_dir.to_str(), Some("/srv/web"));
}

#[test]
fn rejects_invalid_bind_addr() {
    assert!(load(&[("DATABASE_URL", "postgres://x"), ("BIND_ADDR", "nope")]).is_err());
}
