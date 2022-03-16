extern crate diesel;

use self::diesel::pg::PgConnection;
use self::diesel::prelude::*;
use self::diesel::{Insertable, Queryable};
use crate::schema::users;
use crate::schema::users::dsl::*;
use chrono::{NaiveDateTime};
use serde::Serialize;
//use uuid::Uuid;

#[derive(Queryable, Debug, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl UserModel {
    pub fn all_user(conn: &PgConnection) -> Vec<Self> {
        let results = users
            .filter(role.eq("Normal"))
            .limit(5)
            .load::<UserModel>(conn)
            .unwrap();

        results
    }

    pub fn get_user_by_id(user_id: i32, conn: &PgConnection) {
        // unimplimented!
    }

    pub fn create_user(user: CreateUser, conn: &PgConnection) {
        // unimplimented!
    }

    pub fn delete_user(user_id: i32, conn: &PgConnection) -> i32 {
        // unimplimented!
        user_id
    }

    pub fn update_user(user_id: i32, conn: &PgConnection) -> i32 {
        // unimplimented!
        user_id
    }
}
