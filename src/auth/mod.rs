use rocket::request::Outcome;
use rocket::request::Request;
use rocket::request::FromRequest;
use rocket::http::Status;

use rocket_contrib::SerdeError;

use serde_json;

pub mod controller;

use roles::Role;
use roles::RoleDto;

pub const APP_AUTH_COOKIE_NAME: &'static str = "app_auth";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Authorized {
    username: String,
    pub roles: Vec<RoleDto>
}

impl Authorized {
    pub fn new(name: String, roles: Vec<RoleDto>) -> Authorized {
        Authorized {
            username: name,
            roles: roles
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(text) => text,
            Err(_) => String::default(),
        }
    }

    pub fn from_json(txt: &str) -> Option<Authorized> {
        match serde_json::from_str(txt) {
            Ok(auth) => Some(auth),
            Err(_) => None
        }
    }
}

use std::str::FromStr;
impl FromStr for Authorized {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(auth) => Ok(auth),
            Err(_) => Err(()),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Authorized {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        use rocket::Outcome;
        let r = request.cookies()
            .get_private(APP_AUTH_COOKIE_NAME)
            .and_then(|cookie| cookie.value().parse::<Authorized>().ok());
        match r {
            None => {
                return Outcome::Forward(());
            },
            Some(a) => {
                return Outcome::Success(a);
            },
        }
    }
}