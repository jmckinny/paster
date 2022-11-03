#[macro_use]
extern crate rocket;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use paste::Paste;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

use rocket::data::ToByteUnit;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::Data;
mod paste;

pub mod schema;
use diesel::prelude::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new_paste, get_paste])
}
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("pages/main.html");
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
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
    let id = (hasher.finish() as i32).abs();
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
