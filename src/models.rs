use serde::Serialize;
use askama::Template;
use diesel::prelude::*;


#[derive(Template)]
#[template(path = "index.html")]
pub struct MyTemplate {
    pub name: &'static str,
    pub title: &'static str,
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
    pub timers: Vec<Timer>,
    pub title: &'static str,
}

#[derive(Serialize)]
pub struct Hello {
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

