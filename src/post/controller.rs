use rocket::request::LenientForm;
use rocket::response::Redirect;

use diesel;
use diesel::prelude::*;

use DbConn;
use super::model::NewPost;
use schema::posts;

#[post("/posts", data="<new_post>")]
fn new_post(new_post: LenientForm<NewPost>, conn: DbConn) -> Redirect {
    let sql_conn: &SqliteConnection = &conn;
    let post_form = new_post.get();
    diesel::insert_into(posts::table)
        .values(post_form)
        .execute(sql_conn).unwrap();
    Redirect::to("/")
}