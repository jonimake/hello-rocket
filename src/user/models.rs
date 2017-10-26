use schema::users;

/// DO NOT DERIVE FromForm or any kind of deserialization for this.
#[derive(Queryable, Insertable, Debug, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub active: bool,
}

/// Converts UserDTO into a User. Does not set ID to, instead you callee has to explicitly set it
/// after conversion.
impl<'a> From<&'a UserDTO> for User {
    fn from(u: &'a UserDTO) -> User {
        User {
            id: None,
            username: u.username.clone(),
            password: u.password.clone(),
            active: true
        }
    }
}

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct UserDTO {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

impl From<User> for UserDTO {
    fn from(u: User) -> UserDTO {
        UserDTO {
            id: u.id,
            username: u.username,
            password: String::default(),
        }
    }
}

