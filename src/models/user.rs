use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Insertable)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}
