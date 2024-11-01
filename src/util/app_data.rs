use std::error::Error;
use std::fs;

const APP_DATA_DIR_RELATIVE_PATH: &str = ".local/share/arbor";

pub struct AppData {
    language: String,
    thread_count: u8,
    max_suggestion: u8,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            thread_count: 2,
            max_suggestion: 10,
        }
    }
}

impl AppData {
    pub fn build(
        language: Option<String>,
        thread_count: Option<u8>,
        max_suggestion: Option<u8>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut app_data = AppData::default();

        if let Some(lang) = language {
            app_data.language = lang;
        }

        if let Some(thread) = thread_count {
            app_data.thread_count = thread;
        }

        if let Some(max_sugg) = max_suggestion {
            app_data.max_suggestion = max_sugg;
        }

        // Resolve the home directory
        let home_dir = dirs::home_dir().ok_or("Unable to find home directory")?;
        let app_data_dir = home_dir.join(APP_DATA_DIR_RELATIVE_PATH);

        // Create the directory if it doesn't exist
        if !app_data_dir.exists() {
            fs::create_dir_all(&app_data_dir)?;
        }

        Ok(app_data)
    }

    pub fn get_language(&self) -> &str {
        return self.language.as_ref();
    }

    pub fn get_thread_count(&self) -> u8 {
        return self.thread_count;
    }

    pub fn get_max_suggestion(&self) -> u8 {
        return self.max_suggestion;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_app_data() -> Result<(), Box<dyn Error>> {
        let app_data = AppData::build(None, None, None)?;

        assert_eq!(app_data.get_language(), "en-US".to_string());
        assert_eq!(app_data.get_thread_count(), 2);
        assert_eq!(app_data.get_max_suggestion(), 10);

        let app_data = AppData::build(Some("tr-TR".to_string()), Some(4), Some(5))?;

        assert_eq!(app_data.get_language(), "tr-TR".to_string());
        assert_eq!(app_data.get_thread_count(), 4);
        assert_eq!(app_data.get_max_suggestion(), 5);

        let home_dir = dirs::home_dir().unwrap();
        let app_data_dir = home_dir.join(APP_DATA_DIR_RELATIVE_PATH);

        std::fs::remove_dir_all(app_data_dir)?;

        Ok(())
    }
}
