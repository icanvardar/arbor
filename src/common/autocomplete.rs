use std::error::Error;

use crate::util::{app_data::AppData, backup::Backup};

use super::trie::Trie;

pub struct Autocomplete {
    app_data: AppData,
    backup: Option<Backup>,
    trie: Trie,
}

impl Autocomplete {
    pub async fn build(
        language: Option<String>,
        thread_count: Option<u8>,
        max_suggestion: Option<u8>,
        has_backup: bool,
        backup_path: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let app_data = AppData::build(language, thread_count, max_suggestion)?;
        let backup = if has_backup {
            Some(Backup::build(backup_path).await?)
        } else {
            None
        };
        let trie = Trie::new();

        Ok(Self {
            app_data,
            backup,
            trie,
        })
    }

    pub async fn load_backup(&mut self) -> Result<(), Box<dyn Error>> {
        if self.backup.is_some() {
            let backup = self.backup.as_ref().unwrap();

            let backup_data = backup.load_data().await?;

            for word in backup_data {
                Trie::insert(word, &mut self.trie.root, 0)?;
            }
        }

        Ok(())
    }

    pub async fn insert_word(&mut self, word: String) -> Result<(), Box<dyn Error>> {
        Trie::insert(word.clone(), &mut self.trie.root, 0)?;

        if self.backup.is_some() {
            self.backup
                .as_mut()
                .unwrap()
                .save_data(Vec::from([word]))
                .await?;
        }

        Ok(())
    }

    pub async fn suggest_word(&self, prefix: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let suggestions = self.trie.suggest(prefix)?;

        let limit = self.app_data.get_max_suggestion() as usize;

        Ok(suggestions.iter().take(limit).cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[tokio::test]
    async fn it_creates_autocomplete_instance() -> Result<(), Box<dyn Error>> {
        let ac = Autocomplete::build(None, None, None, false, None).await?;

        assert_eq!(ac.app_data.get_language(), "en-US");
        assert!(ac.backup.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn it_loads_backup() -> Result<(), Box<dyn Error>> {
        let backup = Backup::build(None).await?;

        let words = Vec::from(["hello".to_string(), "hi".to_string(), "hey".to_string()]);

        backup.save_data(words.clone()).await?;

        let mut ac = Autocomplete::build(None, None, None, true, None).await?;

        ac.load_backup().await?;

        let word_1 = ac.suggest_word("hell").await?;
        let word_2 = ac.suggest_word("hi").await?;
        let word_3 = ac.suggest_word("hey").await?;

        assert_eq!(
            Vec::from([
                word_1.iter().nth(0).unwrap().to_owned(),
                word_2.iter().nth(0).unwrap().to_owned(),
                word_3.iter().nth(0).unwrap().to_owned(),
            ]),
            words
        );

        std::fs::remove_file(backup.file_path).unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn it_inserts_word_and_suggests() -> Result<(), Box<dyn Error>> {
        let mut ac = Autocomplete::build(None, None, None, false, None).await?;

        let word = "test".to_string();

        ac.insert_word(word.clone()).await?;

        let suggestion = ac.suggest_word(word.as_str()).await?;

        let suggestion = suggestion.iter().nth(0).unwrap().to_owned();

        assert_eq!(word, suggestion);

        Ok(())
    }
}
