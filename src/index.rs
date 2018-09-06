use std::collections::HashMap;

use rocket_contrib::Template;
use DbConn;
use user::models::{User, UserDTO};
use diesel::SqliteConnection;

#[get("/")]
fn show(conn: DbConn) -> Template {
    let sql_conn: &SqliteConnection = &conn;
    let mut ctx: HashMap<&str, Vec<User>> = HashMap::new();
    ctx.insert("users", User::all(sql_conn));
    Template::render("index", ctx)
}
