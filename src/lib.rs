#![feature(custom_derive, plugin, decl_macro, const_fn)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate serde;
extern crate serde_json;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;
#[macro_use] extern crate lazy_static;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::Template;
use diesel::prelude::*;
use dotenv::dotenv;

pub mod database;
pub mod user;
pub mod schema;
pub mod index;
pub mod register;
pub mod roles;
pub mod post;
pub mod auth;
pub mod guards;
pub mod dbconn;
pub use dbconn::DbConn;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

use std::env;
use rocket::Route;
lazy_static! {
    static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not defined");
}
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Initializes a database pool.
pub fn init_pool() -> Pool {
    println!("Database url:{}", DATABASE_URL.to_owned());
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL.to_owned());
    let pool: Pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();
    pool
}

pub fn rocket_with_pool(pool: Pool) -> rocket::Rocket {
    let conn = database::connection::establish_connection();
    let status = conn.execute("select 1");
    println!("{:?}", status);
    let rkt = rocket::ignite()
        .attach(Template::fairing())
        .manage(pool)
        .mount("/",
               get_routes());

    rkt
}

fn get_routes() -> Vec<Route> {
    routes![
       files,
       index::show,
       register::show,
       auth::controller::authenticate,
       post::controller::new_post,
       user::controller::get_user_admin,
       user::controller::get_user,
       user::controller::show_edit_user,
       user::controller::form_edit_user,
       user::controller::new_user]
}

pub fn rocket() -> rocket::Rocket {
    let conn = database::connection::establish_connection();
    let status = conn.execute("select 1");
    println!("{:?}", status);
    let rkt = rocket::ignite()
        .attach(Template::fairing())
        .manage(init_pool())
        .mount("/",
               get_routes());

    rkt
}
/// Static file handler (css and so on)
#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

