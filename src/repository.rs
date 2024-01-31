use crate::models::{Post, NewPost};
use diesel::prelude::*;


pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) {

    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn);
}

