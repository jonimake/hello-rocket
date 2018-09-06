use rocket::request::Outcome;
use rocket::request::Request;
use rocket::request::FromRequest;
use rocket::http::Status;

use auth::APP_AUTH_COOKIE_NAME;
use auth::Authorized;
use roles::RoleDto;


pub struct AdminGuard;


impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        use rocket::Outcome;
        let r = request.cookies()
            .get_private(APP_AUTH_COOKIE_NAME)
            .and_then(|cookie| cookie.value().parse::<Authorized>().ok());
        match r {
            None => {
                Outcome::Forward(())
            },
            Some(a) => {
                if a.roles.contains(&RoleDto::AdminRole) {
                    Outcome::Success(AdminGuard)
                } else {
                    Outcome::Forward(())
                }
            }
        }
    }

}