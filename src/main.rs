#[macro_use]
extern crate rocket;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Read;

use rocket::data::ToByteUnit;
use rocket::Data;
mod paste;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new_paste,get_paste])
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
#[get("/<id>")]
async fn get_paste(id: u64) -> std::io::Result<String>{
    let mut file = std::fs::File::open(format!("pastes/{}", id))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

#[post("/paste/new", data = "<paste>")]
async fn new_paste(paste: Data<'_>) -> std::io::Result<String> {
    let text = paste.open(128.kibibytes()).into_string().await?.to_string();
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let id = hasher.finish();
    let item = paste::Paste::new(
        id,
        text
    );
    item.into_file()?;
    Ok(id.to_string())
}
