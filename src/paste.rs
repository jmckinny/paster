use rocket::tokio::{fs::File, io::AsyncWriteExt};

pub struct Paste {
    id: u64,
    data: String,
}

impl Paste {
    pub fn new(id: u64, data: String) -> Self {
        Paste { id, data }
    }

    pub async fn into_file(self) -> Result<(), std::io::Error> {
        let mut file = File::create(format!("pastes/{}", self.id)).await?;
        file.write_all(&self.data.bytes().collect::<Vec<u8>>())
            .await?;
        Ok(())
    }
}
