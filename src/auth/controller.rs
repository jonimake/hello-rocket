use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;

use rocket_contrib::Template;

use diesel;
use diesel::prelude::*;

use DbConn;
use schema::users;
use schema::user_roles;
use schema::roles;

use super::Authorized;

use user::models::User;
use roles::{Role, UserRole};
use roles::RoleDto;

#[derive(FromForm, Deserialize)]
struct LoginData {
    username: String,
    password: String
}

#[post("/login", data="<login_form>")]
fn authenticate(login_form: Form<LoginData>, mut c: Cookies, conn: DbConn) -> Redirect {
    use schema::user_roles::dsl::*;
    use schema::users::dsl::*;
    use schema::roles::dsl::*;

    let sql_conn: &SqliteConnection = &conn;
    let user = login_form.into_inner();

    let qresult: Result<User, _> = users.filter(username.eq(user.username)).filter(password.eq(user.password)).first(sql_conn);
    let _ = match qresult {
        Ok(user) => {

            let role_ids:Vec<i32> = UserRole::belonging_to(&user)
                .select(role_id).load(sql_conn).unwrap();

            let all_roles: Vec<Role> = roles.load(sql_conn).unwrap();

            let current_user_roles  = all_roles.into_iter()
                .filter(|rr|role_ids.iter().any(|i|rr.id == *i))
                .collect::<Vec<Role>>();


            let role_dtos: Vec<RoleDto> = current_user_roles.iter().map(|r| RoleDto::from(r.clone())).collect();
            let auth = Authorized::new(user.username.clone(), role_dtos);
            println!("Authenticated ok: {}", auth.to_json());
            let ck = Cookie::new(super::APP_AUTH_COOKIE_NAME, auth.to_json());
            c.add_private(ck);
            true
        },
        _ => false
    };
    Redirect::to("/")
}

