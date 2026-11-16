//! Écrit le document OpenAPI sur la sortie standard :
//! `cargo run -p app --bin export-openapi > openapi.json`

fn main() -> Result<(), serde_json::Error> {
    println!("{}", app::http::openapi::spec().to_pretty_json()?);
    Ok(())
}
