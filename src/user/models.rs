use serde;
use serde_json;
use schema::users;
use roles::RoleDto;
//use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use diesel::backend::Backend;
use diesel::sqlite::Sqlite;
use diesel::connection::AnsiTransactionManager;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use DbConn;

/// DO NOT DERIVE FromForm or any kind of deserialization for this.
#[derive(Queryable, Debug, AsChangeset, Identifiable, Serialize, Clone, Default)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub active: bool
}

impl User {
    pub fn all(conn: &SqliteConnection) -> Vec<User> {
        use schema::users;
        let all_users: Vec<User> = users::table.load::<User>(conn).expect("Error loading users");
        all_users
    }

    pub fn get(id: i32, conn: DbConn) -> User {
        use schema::users;
        use schema::user_roles;
        use schema::roles;
        let sqlconn: &SqliteConnection = &conn;

        let user: User = users::table.find(id).first(sqlconn).unwrap();


        let roles_query = user_roles::table
            .inner_join(roles::table)
            .inner_join(users::table)
            .filter(user_roles::user_id.eq(id))
            .select(roles::all_columns);

        use roles::Role;
        let roles: Vec<Role> = roles_query.load(sqlconn).expect("Could not query user data");
        user
    }
}


impl<'a> From<&'a UpdateUser> for User {
    fn from(u: &'a UpdateUser) -> User {
        User {
            id: u.id,
            username: u.username.clone(),
            password: u.password.clone(),
            active: true,
            //      roles: None
        }
    }
}




#[derive(FromForm, Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub active: bool
}



#[derive(Serialize, Deserialize, Debug)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub active: bool,
    pub roles: Vec<RoleDto>
}

impl From<User> for UserDTO {
    fn from(u: User) -> UserDTO {
        UserDTO {
            id: u.id,
            username: u.username.clone(),
            password: u.password.clone(),
            active: true,
            roles: Vec::default()
        }
    }
}

#[derive(FromForm, Deserialize, Debug, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub active: Option<bool>,
}
