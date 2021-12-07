// password hashing
extern crate bcrypt;
use bcrypt:: {
  DEFAULT_COST,
  hash,
  verify
};
// json serialization
use serde:: {
  Serialize,
  Deserialize
};
// random unique id
use uuid::Uuid;
// time
use chrono:: {
  DateTime,
  Utc
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  #[serde(rename = "_id")]
  pub id: Uuid,
  pub username: String,
  pub password: String,
  pub email: String,
  pub role: String,
  pub created: DateTime < Utc >,
  pub updated: DateTime < Utc >
}

impl User {
  pub fn new(username: String, email: String, password: String) -> Self {
    let hashed_password = hash(&password, DEFAULT_COST).unwrap();
    let new_user = User {
      id: Uuid::new_v4(),
      username: username,
      password: hashed_password,
      email: email,
      role: String::from("Normal"),
      created: Utc::now(),
      updated: Utc::now(),
    };
    new_user
  }
  
  pub fn from_insertable(user: InsertableUser) -> Self {
    User::new(user.username, user.password, user.email)
  }
  
  pub fn match_password(&self, password: &String) -> bool {
    let is_match = verify(&self.password, password).unwrap();
    is_match
  }
  
  pub fn update_password(&mut self, password: &String) {
    let new_hashed_password = hash(&password, DEFAULT_COST).unwrap();
    self.password = new_hashed_password;
    self.updated = Utc::now();
  }
  
  pub fn update_user(&mut self, username: &String, email: &String) {
    self.username = username.to_string();
    self.email = email.to_string();
    self.updated = Utc::now();
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
  pub username: String,
  pub password: String,
  pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
  id: String,
  username: String,
  email: String,
}

impl ResponseUser {
  pub fn from_user(user: &User) -> Self {
    ResponseUser {
      id: user.id.to_string(),
      username: format!("{}",user.username),
      email: format!("{}",user.email),
    }
  }
}

pub trait UserTrait {
  fn find_by_id(id: &str) -> User;
}