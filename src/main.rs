mod db;
mod rest;

use axum::{
    routing::get,
    response::Json,
    Router,
    response::sse::{Event, Sse},
    Extension,
};
use anyhow::Result;
use sqlx::SqlitePool;
use sqlx::migrate::Migrator;
use serde::Serialize;
use askama::Template;
use maud::{html, Markup};
use axum_extra::{headers, TypedHeader};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;


#[derive(Template)]
#[template(path = "index.html")]
pub struct MyTemplate {
    name: &'static str,
    title: &'static str,
}

async fn handle_main() -> MyTemplate {
    MyTemplate{
        name: "Ricardo", 
        title: "This is test of HTMX"
    }
}


async fn handle_maud() -> Markup {
    html! {
        h1 { "Hello world Maud!" }
    }
}

#[derive(Template)]
#[template(path="elements/timer.html")]
pub struct Timer {
    pub start : f64,
    pub end : f64,
    pub step : f64,
}

#[derive(Template)]
#[template(path="timer_table.html")]
pub struct TimerTable {
    timers: Vec<Timer>,
    title: &'static str,
}

async fn handle_table() -> TimerTable {
    let t1 = Timer{start: 0.0, end: 10.0, step: 1.0};
    let t2 = Timer{start: 100.0, end: 1000.0, step: 10.0};
    let timers = vec![t1, t2];
    TimerTable{timers, title: "Hola mundo"}
}

#[derive(Serialize)]
struct Hello {
    name: String,
}

async fn returns_json() -> Json<Hello> {
    let hello = Hello {
        name: String::from("world"),
    };
    Json(hello)
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

static MIGRATOR: Migrator = sqlx::migrate!();


pub async fn init_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = SqlitePool::connect(&database_url).await?;
    MIGRATOR.run(&connection_pool).await?;
    Ok(connection_pool)
}

#[tokio::main]
async fn main() -> Result<()> {

    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await?;

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/maud", get(handle_maud))
        .route("/ajax", get(returns_json))
        .route("/sse", get(sse_handler))
        .route("/table", get(handle_table))
        .nest_service("/rest", rest::books_service())
        .layer(Extension(connection_pool))
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

