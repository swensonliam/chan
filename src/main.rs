#[macro_use] extern crate rocket;

#[macro_use] extern crate diesel;

pub mod models;
pub mod schema;

use crate::models::*;
use crate::schema::*;

use diesel::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use serde::Serialize;
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
    let boards: Vec<Board> = boards::table
        .select(boards::all_columns)
        .load::<Board>(&crate::establish_connection())
        .expect("Failed to fetch boards");

    let mut context = HashMap::new();
    context.insert("boards", boards);

    Template::render("home", context)
}

#[get("/<path>")]
async fn board(path: &str) -> Template {
    let connection = &crate::establish_connection();

    let matching_boards = boards::table
        .select(boards::all_columns)
        .filter(boards::path.eq(path))
        .load::<Board>(connection)
        .expect("Failed to fetch board");
    let board = matching_boards.first()
        .expect("Failed to find board");

    let posts = posts::table
        .select(posts::all_columns)
        .filter(posts::board.eq(path))
        .load::<Post>(connection)
        .expect("Failed to fetch posts");

    #[derive(Serialize, Debug)]
    enum ContextValue<'a> {
        ContextBoard(&'a Board),
        ContextPosts(Vec<Post>)
    }

    let mut context = HashMap::new();
    context.insert("board", ContextValue::ContextBoard(board));
    context.insert("posts", ContextValue::ContextPosts(posts));

    return Template::render("board", context)
}

#[derive(FromForm)]
struct PostForm<'r> {
    r#text: &'r str,
}

#[post("/<path>/post", data = "<data>")]
fn post(path: &str, data: Form<PostForm<'_>>) -> Redirect {
    let connection = &crate::establish_connection();

    let post = NewPost {
        board: path.to_string(),
        text: data.text.to_string(),
    };

    diesel::insert_into(posts::table)
        .values(post)
        .execute(connection)
        .expect("Failed to post");

    Redirect::to(uri!(board(path)))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![home, board, post])
        .attach(Template::fairing())
        .launch()
        .await;
}
