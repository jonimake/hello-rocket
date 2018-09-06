extern crate rocket;
extern crate rocket_contrib;
extern crate hello_rocket_lib;
extern crate dotenv;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate parking_lot;
extern crate diesel_migrations as migrations;

use std::env;

use self::hello_rocket_lib::user::models::*;
use self::rocket::local::Client;
use self::rocket::http::{Status, ContentType};

use std::io;
use self::hello_rocket_lib::database::connection::establish_connection as get_connection;
use self::parking_lot::Mutex;
use std::process::Command;

const DB_URL: &'static str = "test.db";
static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! db_setup {
    ( $code:block ) => {{

        let _lock = DB_LOCK.try_lock_for(::std::time::Duration::new(30, 0));
        Command::new("rm")
            .arg(format!{"{}",DB_URL})
            .status()
            .expect("failed to delete test.db");

        env::set_var("DATABASE_URL", DB_URL);

        let connection = get_connection();
        let _ = migrations::setup_database(&connection);
        let migrations_dir = migrations::find_migrations_directory().expect("could not find migrations directory");
        let _ = migrations::run_pending_migrations_in_directory(
            &connection,
            &migrations_dir,
            &mut io::sink(),
        ).unwrap();

        $code;
    }}
}


#[test]
fn hello_world() {

    db_setup!({
        let r: self::rocket::Rocket = hello_rocket_lib::rocket();
        let client = Client::new(r).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    })
}

#[test]
fn test_register() {
    db_setup!({
        let r: self::rocket::Rocket = hello_rocket_lib::rocket();
        let client = Client::new(r).expect("valid rocket instance");
        let data = format!("username={}&password={}", "joni", "testisalasana");
        let response = client.post("/users")
            .header(ContentType::Form)
            .body(&data)
            .dispatch();
        assert_eq!(response.status(), Status::SeeOther);

        let users = User::all(&get_connection());
        assert_eq!(users.len(), 1);

        assert_eq!(users.first().unwrap().username, "joni");
    })
}