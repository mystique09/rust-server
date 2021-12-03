
pub struct User {
    username: String,
    password: String,
    role: String
}

impl User {
    pub fn new(username: String, password: String) -> User{
        User {
            username: username,
            password: password,
            role: String::from("Normal")
        }
    }
}

pub struct CreateUser {
    username: String,
    password: String
}

pub trait UserTrait {
    fn find_by_id(id: &str) -> User;
}