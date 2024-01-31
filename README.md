# Rust + HTMX

- https://github.com/ThePrimeagen/more-htmx-eploration
- https://github.com/thebracket/webinar_axumcrud
- https://medium.com/@qkpiot/building-a-robust-rust-backend-with-axum-diesel-postgresql-and-ddd-from-concept-to-deployment-b25cf5c65bc8

## Notes

- struct internal --> to askama (template html/css)
- struct internal --> to database (diesel ORM)
- struct internal --> to public proto messages (for let RPC)
- method internal --> to public proto service (for let RPC)

## Builds depends

cargo install cargo-watch


# Driver Sqlite3

download https://www.sqlite.org/2024/sqlite-dll-win-x64-3450100.zip

Acrimos un cmd de VS2022
%comspec% /K "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" amd64

Generamos la .lib:
lib /def:sqlite3.def /out:sqlite3.lib

Poner variable de entorno
set SQLITE3_LIB_DIR=D:\dev\rust-htmx\depends\sqlite-dll-win-x64-3450100

instalar diesel_cli con la feature de sqlite

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
. cargo add sqlx --features runtime-tokio --features sqlite 
- cargo add dotenv 
- cargo add anyhow 
- cargo add once_cell 
- cargo add dotenv 
- cargo add anyhow 
- cargo add once_cell 
- cargo add diesel --features sqlite 
- cargo install diesel_cli --no-default-features --features sqlite 
- DATABASE_URL="sqlite://sqlite.db" > .env
- diesel setup 
- diesel migration generate create_posts 
- diesel migration run 
- cargo add rusqlite 
- cargo add diesel_migrations 

cargo add deadpool-diesel --features sqlite 
cargo add uuid7 --features serde 
cargo add uuid --features serde --features v4 --features fast-rng 
