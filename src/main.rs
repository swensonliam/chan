#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod models;
pub mod schema;

use crate::models::*;
use crate::schema::*;

use diesel::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket_dyn_templates::Template;
use std::{env, collections::HashMap};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
async fn home() -> Template {
    let mut context = HashMap::new();

    let boards: Vec<Board> = boards::table
        .select(boards::all_columns)
        .load::<Board>(&crate::establish_connection())
        .expect("Failed to fetch boards");

    context.insert("boards", boards);

    Template::render("home", context)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![home])
        .attach(Template::fairing())
        .launch()
        .await;
}
