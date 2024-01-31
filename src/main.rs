pub mod db;
pub mod rest;
pub mod models;
pub mod views;
pub mod connection;
pub mod schema;
pub mod repository;

use axum::{
    routing::get,
    Router,
    // Extension,
};
use anyhow::Result;
// use sqlx::SqlitePool;
// use sqlx::migrate::Migrator;
use views::{handle_main, handle_maud, returns_json, sse_handler, handle_table};
use self::models::*;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};


// Define embedded database migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");


// static MIGRATOR: Migrator = sqlx::migrate!();


// pub async fn init_db() -> Result<SqlitePool> {
//     let database_url = std::env::var("DATABASE_URL")?;
//     let connection_pool = SqlitePool::connect(&database_url).await?;
//     MIGRATOR.run(&connection_pool).await?;
//     Ok(connection_pool)
// }

#[tokio::main]
async fn main() -> Result<()> {

    // Load environment variables from .env if available
    // dotenv::dotenv().ok();
    //
    use self::schema::posts::dsl::*;

    tracing_subscriber::fmt::init();

    let connection_pool = &mut connection::establish_connection();

    repository::create_post(connection_pool, "hola", "amigo");
    
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection_pool)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
        println!("{}", post.published);
    }

    // Initialize the database and obtain a connection pool
    // let connection_pool = init_db().await?;

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/maud", get(handle_maud))
        .route("/ajax", get(returns_json))
        .route("/sse", get(sse_handler))
        .route("/table", get(handle_table))
        .nest_service("/rest", rest::books_service())
        // .layer(Extension(connection_pool))
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

