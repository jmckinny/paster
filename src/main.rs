#[macro_use]
extern crate rocket;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use paste::Paste;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rocket::data::ToByteUnit;
use rocket::Data;
mod paste;

pub mod schema;
use diesel::prelude::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new_paste, get_paste])
}
#[get("/")]
fn index() -> &'static str {
    "
    USAGE
      POST /paste/new
          accepts raw data in the body of the request and responds with the id of the paste
      GET /<id>
          retrieves the content for the paste with id `<id>`
    "
}
#[get("/<query_id>")]
fn get_paste(query_id: u64) -> std::io::Result<String> {
    use self::schema::posts::dsl::*;

    let mut conn = establish_connection();
    let result = diesel::QueryDsl::filter(posts, id.eq(query_id as i32))
        .load::<Paste>(&mut conn)
        .expect("Failed to read from DB");
    Ok(result[0].get_data())
}

#[post("/paste/new", data = "<paste>")]
async fn new_paste(paste: Data<'_>) -> std::io::Result<String> {
    use crate::schema::posts;

    let text = paste.open(128.kibibytes()).into_string().await?.to_string();
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let id = hasher.finish() as i32;
    let item = paste::Paste::new(id, text);
    let mut conn = establish_connection();
    diesel::insert_into(posts::table)
        .values(item)
        .execute(&mut conn)
        .expect("Failed to create post");
    Ok(id.to_string())
}

fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
    SqliteConnection::establish(&database_url).expect("Failed to connect to database")
}
