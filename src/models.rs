use serde::Serialize;
use diesel::{Queryable, Insertable};

use crate::schema::*;

#[derive(Debug, Serialize, Queryable)]
pub struct Board {
    pub id: i32,
    pub path: String,
    pub name: String,
}

#[derive(Debug, Serialize, Queryable, Insertable)]
pub struct Post {
    pub id: i32,
    pub board: String,
    pub text: String,
}

#[table_name="posts"]
#[derive(Debug, Insertable)]
pub struct NewPost {
    pub board: String,
    pub text: String,
}
