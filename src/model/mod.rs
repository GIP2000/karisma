use crate::*;
use karisma_derive::DBTraits;

#[derive(DBTraits)]
#[model(User)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub posts: Vec<Post>,
}

pub enum UserEnumTypes {
    Id(u32),
    FirstName(String),
    LastName(String),
    Posts(PostEnumTypes),
}

#[derive(DBTraits)]
#[model(Post)]
pub struct Post {
    pub id: u32,
    pub content: String,
    pub user_id: u32,
    pub user: User,
}
pub enum PostEnumTypes {
    Id(u32),
    UserId(u32),
    Content(String),
    User(User),
}

pub enum Models {
    User(Vec<crate::filter::Where<UserEnumTypes>>),
    Post(Vec<crate::filter::Where<PostEnumTypes>>),
}
