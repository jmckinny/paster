use rocket::tokio::{fs::File, io::AsyncWriteExt};

pub struct Paste {
    id: i32,
    body: String,
}

impl Paste {
    pub fn new(id: i32, data: String) -> Self {
        Paste { id, body: data }
    }

    pub async fn into_file(self) -> Result<(), std::io::Error> {
        let mut file = File::create(format!("pastes/{}", self.id)).await?;
        file.write_all(&self.body.bytes().collect::<Vec<u8>>())
            .await?;
        Ok(())
    }
}
