#[macro_use]
extern crate rocket;
use rocket::tokio::io::AsyncReadExt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rocket::data::ToByteUnit;
use rocket::Data;
mod paste;
use rocket::fs::{relative, FileServer};
use rocket::tokio::fs::File;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![new_paste, get_paste])
        .mount("/", FileServer::from(relative!("static")))
}

#[get("/<query_id>")]
async fn get_paste(query_id: i32) -> std::io::Result<String> {
    let mut file = File::open(format!("pastes/{}", query_id)).await?;
    let mut paste = String::new();
    file.read_to_string(&mut paste).await?;
    Ok(paste)
}

#[post("/paste/new", data = "<paste>")]
async fn new_paste(paste: Data<'_>) -> std::io::Result<String> {
    let text = paste.open(128.kibibytes()).into_string().await?.to_string();
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let id = (hasher.finish() as i32).abs();
    let item = paste::Paste::new(id, text.clone());
    item.into_file().await?;
    Ok(format!("{}", id))
}
