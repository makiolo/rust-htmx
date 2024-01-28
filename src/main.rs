use axum::{
    routing::get,
    Router
};
use askama::Template;
use maud::{html, Markup};


#[derive(Template)]
#[template(path = "index.html")]
pub struct MyTemplate {
    name: &'static str,
    title: &'static str,
}

async fn handle_main() -> MyTemplate {
    return MyTemplate{
        name: "Ricardo", 
        title: "This is test of HTMX"
    };
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
    let mut timers = Vec::new();
    timers.push(t1);
    timers.push(t2);
    return TimerTable{timers, title: "Hola mundo"};
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/maud", get(handle_maud))
        .route("/table", get(handle_table));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

