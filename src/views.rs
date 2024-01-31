use anyhow::Result;
use maud::{html, Markup};
use axum_extra::{headers, TypedHeader};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;
use axum::{
    response::Json,
    response::sse::{Event, Sse},
};
use crate::models::{MyTemplate, Timer, TimerTable, Hello};


// pub mod views {

pub async fn handle_main() -> MyTemplate {
    MyTemplate {
        name: "Ricardo", 
        title: "This is test of HTMX"
    }
}


pub async fn handle_maud() -> Markup {
    html! {
        h1 { "Hello world Maud!" }
    }
}


pub async fn handle_table() -> TimerTable {
    let t1 = Timer{start: 0.0, end: 10.0, step: 1.0};
    let t2 = Timer{start: 100.0, end: 1000.0, step: 10.0};
    let timers = vec![t1, t2];
    TimerTable{timers, title: "Hola mundo"}
}


pub async fn returns_json() -> Json<Hello> {
    let hello = Hello {
        name: String::from("world"),
    };
    Json(hello)
}


pub async fn sse_handler(TypedHeader(user_agent): TypedHeader<headers::UserAgent>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> 
{
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

// }
