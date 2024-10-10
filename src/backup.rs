use std::error::Error;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

const BACKUP_FILE_PATH: &str = "file";

pub struct Backup {
    file: Arc<Mutex<File>>,
}

impl Backup {
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(BACKUP_FILE_PATH)
            .await
            .expect("Could not open file");

        Ok(Self {
            file: Arc::new(Mutex::new(file)),
        })
    }

    pub async fn save_data(&self, words: Vec<String>) -> Result<(), Box<dyn Error>> {
        let encoded_words = bincode::serialize(&words).expect("Could not encode");

        let mut file = self.file.lock().await;
        file.write_all(&encoded_words)
            .await
            .expect("Could not write to file");
        file.write_all(b"\n")
            .await
            .expect("Could not write to file");

        Ok(())
    }

    pub async fn load_data(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut file = self.file.lock().await;

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .await
            .expect("Could not read file!");
        println!("contents: {:#?}", contents);

        let lines = contents.split(|&byte| byte == b'\n');

        let mut flattenend_v = Vec::new();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let decoded_v: Vec<String> =
                bincode::deserialize(line).expect("Could not decodee vector");
            flattenend_v.extend(decoded_v);
        }

        Ok(flattenend_v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn init_backup() -> Backup {
        return Backup::build().await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_backup_file() {
        let backup = init_backup().await;
        let file = backup.file.lock().await;

        assert!(file.metadata().await.unwrap().is_file());
    }

    #[tokio::test]
    async fn it_saves_data() {
        let backup = init_backup().await;

        let words = Vec::from([
            "hello".to_string(),
            "hellium".to_string(),
            "hundred".to_string(),
        ]);

        backup.save_data(words).await.unwrap();

        let result = backup.load_data().await.unwrap();

        println!("{:?}", result);
    }
}
