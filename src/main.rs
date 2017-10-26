#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

use rocket_contrib::Template;
use diesel::connection::Connection;
use dotenv::dotenv;

pub mod database;
pub mod user;
pub mod schema;
pub mod index;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

// An alias to the type for a pool of Diesel SQLite connections.
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    r2d2::Pool::new(config, manager).expect("db pool")
}

fn rocket() -> rocket::Rocket {
    let conn = database::connection::establish_connection();

    embed_migrations!("migrations");
    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).expect("Database migration failed");


    let status = conn.execute("select 1");
    println!("{:?}", status);
    let rkt = rocket::ignite()
        .attach(Template::fairing())
        .manage(init_pool())
        .mount("/",
               routes![
               index::show,
               user::controller::get_user,
               user::controller::show_edit_user,
               user::controller::form_edit_user,
               user::controller::new_user]);
    
    rkt
}

fn main() {
    dotenv().ok();
    rocket().launch();

}


use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}