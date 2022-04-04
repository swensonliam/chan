use serde::Serialize;
use diesel::Queryable;

#[derive(Queryable, Serialize)]
pub struct Board {
    pub id: i32,
    pub path: String,
    pub name: String,
}