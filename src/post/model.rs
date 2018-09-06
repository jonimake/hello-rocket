use schema::posts;
use user::models::User;

#[derive(Queryable, Identifiable, Associations)]
#[table_name="posts"]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, FromForm)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}