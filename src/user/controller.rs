use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, LenientForm};
use rocket::response::Redirect;
use rocket::http::Status;
use rocket_contrib::Template;

//use diesel;
//use diesel::debug_query;
//use diesel::prelude::*;

use DbConn;
use super::models::{UserDTO, User, NewUser, UpdateUser};

//use schema::users;


use roles::UserRole;

use auth::Authorized;
use auth;
use roles::RoleDto;

use guards::AdminGuard;

#[post("/users", data="<new_user>")]
fn new_user(new_user: LenientForm<NewUser>, conn: DbConn) -> Redirect {
    let sql_conn: &SqliteConnection = &conn;
    let mut user_form = new_user.into_inner();
    user_form.active = true;
    diesel::insert_into(users::table).values(&user_form)
        .execute(sql_conn).unwrap();
    Redirect::to("/")
}

#[get("/users/<id>", rank = 1)]
fn get_user_admin(_auth: Authorized, id: i32, conn: DbConn) -> Template {
    //use schema::users::dsl::*;
    let sqlconn: &SqliteConnection = &conn;
    let user: User = users::table.find(id).first(sqlconn).unwrap();
    //let user: User = users.find(id).first(sqlconn).unwrap();

    //use schema::roles;
    //use schema::user_roles;


    let sql2 = user_roles::table
        .inner_join(roles::table)
        .inner_join(users::table)
        .filter(user_roles::user_id.eq(id))
        .select(roles::all_columns);

    use roles::Role;
    let roles: Vec<Role> = sql2.load(sqlconn).expect("Could not query user data");

    let mut dto: UserDTO = UserDTO::from(user);
    dto.roles = roles.into_iter().map(|role| {RoleDto::from(role)}).collect();
    Template::render("user", dto)
}

#[get("/users/<id>", rank = 2)]
fn get_user(id: i32, conn: DbConn) -> Template {
    //use schema::users::dsl::*;
    /*
    let sqlconn: &SqliteConnection = &conn;
    let user: User = users::table.find(id).first(sqlconn).unwrap();
    //let user: User = users.find(id).first(sqlconn).unwrap();

    use schema::roles;
    use schema::user_roles;
    use diesel::sqlite::Sqlite;


    let sql2 = user_roles::table
        .inner_join(roles::table)
        .inner_join(users::table)
        .filter(user_roles::user_id.eq(id))
        .select(roles::all_columns);

    use roles::Role;
    let roles: Vec<Role> = sql2.load(sqlconn).expect("Could not query user data");

    */
    User::get(id, conn);
    let mut dto: UserDTO = UserDTO::from(user);
    dto.roles = roles.into_iter().map(|role| {RoleDto::from(role)}).collect();
    Template::render("user", dto)
}


#[get("/users/<id>/edit")]
fn show_edit_user(id: i32, conn: DbConn) -> Template {
    let sql_conn: &SqliteConnection = &conn;
    let u: User = users::table.find(id).first(sql_conn).unwrap();
    Template::render("edit_user", u)
}

#[post("/users/<id_param>/edit", data="<edit_user>")]
fn form_edit_user(_auth_guard: Authorized, id_param: i32, edit_user: Form<UpdateUser>, conn: DbConn) -> Result<Redirect, Status> {
    println!("{:?}", _auth_guard);
    let sql_conn: &SqliteConnection = &conn;

    let user_form = edit_user.get();
    let user: User = User::from(user_form);
    if id_param != user.id {
        return Err(Status::BadRequest);
    }
    println!("{:?}", user);
    let sql = diesel::update(&user)
        .set(&user);
    //let debug = debug_query::<SqliteConnection, _>(&sql);
    //println!("{:?}", debug.to_string());
    sql.execute(sql_conn).expect("Could not update user");

    Ok(Redirect::to(&format!("/users/{}", id_param)))
}