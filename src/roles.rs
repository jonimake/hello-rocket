use schema::roles;
use schema::user_roles;
use schema::users;
use user::models::User;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug, AsChangeset, Identifiable, Associations, Deserialize, Serialize)]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub role: String
}

use diesel::SqliteConnection;
impl Role {
    pub fn get_by_user(u: &User, conn: &SqliteConnection) -> Vec<Role> {
        let sql = user_roles::table
            .inner_join(roles::table)
            .inner_join(users::table)
            .filter(user_roles::user_id.eq(u.id))
            .select(roles::all_columns);

        let roles: Vec<Role> = sql.load(conn).expect("Could not query user data");
        roles
    }
}

#[derive(Clone, Copy, Queryable, Insertable, Debug, AsChangeset, Identifiable, Associations)]
#[table_name = "user_roles"]
#[belongs_to(User)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum RoleDto {
    UserRole,
    AdminRole
}


impl From<Role> for RoleDto {
    fn from(role: Role) -> RoleDto {
        match role.role.to_lowercase().as_ref() {
            "admin" => RoleDto::AdminRole,
            _ => RoleDto::UserRole
        }
    }
}

impl<'a> From<&'a Role> for RoleDto {
    fn from(role: &Role) -> RoleDto {
        match role.role.to_lowercase().as_ref() {
            "admin" => RoleDto::AdminRole,
            _ => RoleDto::UserRole
        }
    }
}