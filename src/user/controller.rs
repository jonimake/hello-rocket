use rocket::request::LenientForm;
use rocket::response::Redirect;
use rocket_contrib::Template;
use diesel;
use diesel::prelude::*;

use DbConn;
use super::models::{UserDTO, User};
use schema::users;
use schema::users::dsl::*;

#[post("/users", data="<new_user>")]
fn new_user(new_user: LenientForm<UserDTO>, conn: DbConn) -> Redirect {
    let sql_conn: &SqliteConnection = &conn;
    let user_form = new_user.get();
    let user: User = User::from(user_form);
    diesel::insert(&user).into(users::table)
    .execute(sql_conn).unwrap();
    Redirect::to("/")
}

#[get("/users/<user_id>")]
fn get_user(user_id: i32, conn: DbConn) -> Template {
    let sql_conn: &SqliteConnection = &conn;
    let u: User = users.find(user_id).first(sql_conn).unwrap();
    Template::render("user", UserDTO::from(u))
}


#[get("/users/<user_id>/edit")]
fn show_edit_user(user_id: i32, conn: DbConn) -> Template {
    let sql_conn: &SqliteConnection = &conn;
    let u: User = users.find(user_id).first(sql_conn).unwrap();
    Template::render("edit_user", UserDTO::from(u))
}

#[post("/users/<user_id>/edit", data="<edit_user>")]
fn form_edit_user(user_id: i32, edit_user: LenientForm<UserDTO>, conn: DbConn) -> Redirect {
    let sql_conn: &SqliteConnection = &conn;

    let user_form = edit_user.get();
    let mut user: User = User::from(user_form);
    user.id = Some(user_id);
    println!("Updating user {} = {:?}", user_id, user);
    diesel::update(users::table)
        .set(&user)
        .execute(sql_conn).expect("Could not update user");

    Redirect::to(&format!("/users/{}", user_id))
}