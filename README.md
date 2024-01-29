# Rust + HTMX

- https://github.com/ThePrimeagen/more-htmx-eploration
- https://github.com/thebracket/webinar_axumcrud

## Notes

- struct internal --> to askama (template html/css)
- struct internal --> to database (diesel ORM)
- struct internal --> to public proto messages (for let RPC)
- method internal --> to public proto service (for let RPC)

## Builds depends

cargo install cargo-watch

# Depends

- cargo add axum 
- cargo add tokio --features full 
- cargo add clap --features derive 
- cargo add serde --features derive 
- cargo add serde_json 
- cargo add askama --features serde --features markdown --features with-axum --features serde-json 
- cargo add askama_axum
- cargo add axum_server 
- cargo add tracing 
- cargo add tracing-subscriber --features env-filter 
- cargo add log 
- cargo add lazy_static 
- cargo add maud --features axum 
- cargo add sqlx --features runtime-tokio --features sqlite 
- cargo add dotenv 
- cargo add anyhow 
- cargo add once_cell 
