use std::error::Error;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

// TODO: find out how to switch back to this path => "/var/lib/arbor/backup"
// in production mode
const BACKUP_FILE_PATH: &str = "/tmp/arbor/backup";

pub struct Backup {
    pub file_path: String,
    file: Arc<Mutex<File>>,
}

impl Backup {
    pub async fn build(file_path: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let path = std::path::Path::new(match file_path {
            Some(path) => path,
            None => BACKUP_FILE_PATH,
        });

        if !path.exists() {
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
        }

        let file = OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(path)
            .await
            .expect("Could not open file");

        Ok(Self {
            file_path: path.to_str().unwrap().to_owned(),
            file: Arc::new(Mutex::new(file)),
        })
    }

    pub async fn save_data(&self, words: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut file = self.file.lock().await;

        for word in words {
            file.write_all(word.as_bytes()).await?;
            file.write_all(b"\n").await?;
        }

        Ok(())
    }

    pub async fn load_data(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut file = self.file.lock().await;

        file.seek(std::io::SeekFrom::Start(0)).await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn init_backup(file_path: Option<&str>) -> Backup {
        return Backup::build(file_path).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_backup_file() {
        let backup = init_backup(Some("file")).await;
        let file = backup.file.lock().await;

        assert!(file.metadata().await.unwrap().is_file());
    }

    #[tokio::test]
    async fn it_saves_data() {
        let backup = init_backup(Some("file")).await;

        let words = Vec::from([
            "hello".to_string(),
            "hellium".to_string(),
            "hundred".to_string(),
        ]);

        backup.save_data(words.clone()).await.unwrap();

        let data_from_file = backup.load_data().await.unwrap();

        assert_eq!(data_from_file, words);

        std::fs::remove_file(backup.file_path).unwrap();
    }
}
