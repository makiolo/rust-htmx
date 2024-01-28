# Rust + HTMX

## Builds depends

cargo install cargo-watch

# Depends

cargo add axum 
cargo add tokio --features full 
cargo add clap --features derive 
cargo add serde --features derive 
cargo add serde_json 
cargo add askama --features serde --features markdown --features with-axum --features serde-json 
cargo add askama_axum
cargo add axum_server 
cargo add tracing 
cargo add tracing-subscriber --features env-filter 
cargo add log 

cargo add lazy_static 
