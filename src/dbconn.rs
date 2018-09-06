use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use Pool;

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

    fn deref(&self) -> &SqliteConnection {
        &self.0
    }
}

impl AsRef<SqliteConnection> for DbConn {
    fn as_ref(&self) -> &SqliteConnection {
        &self.0
    }
}

use std::borrow::Borrow;
impl Borrow<SqliteConnection> for DbConn {
    fn borrow(&self) -> &SqliteConnection {
        &self.0
    }
}

impl<'a, 'b: 'a> Into<&'a SqliteConnection> for &'b DbConn {
    fn into(self) -> &'a SqliteConnection {
        &self.0
    }
}