use serde::{
  Serialize,
  Deserialize
};
use uuid::Uuid;
use argon2;
use rand::{thread_rng, Rng};
use rand::distribution::Alphanumeric;
use chrono::{DateTime, Utc};

pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String
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