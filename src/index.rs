use std::collections::HashMap;

use rocket_contrib::Template;
use DbConn;
use user::models::{User, UserDTO};
use diesel::SqliteConnection;
use schema::users::dsl::*;
use diesel::LoadDsl;

#[get("/")]
fn show(conn: DbConn) -> Template {
    let sql_conn: &SqliteConnection = &conn;
    let all_users: Vec<UserDTO> = users.load::<User>(sql_conn).unwrap()
        .into_iter().map(|u| UserDTO::from(u)).collect();;
    let mut ctx: HashMap<&str, Vec<UserDTO>> = HashMap::new();
    ctx.insert("users", all_users);
    Template::render("index", ctx)
}