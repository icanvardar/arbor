use std::{error::Error, pin::Pin};

use clap::Parser;

use super::autocomplete::Autocomplete;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    language: Option<String>,

    #[arg(short, long)]
    thread_count: Option<u8>,

    #[arg(short, long)]
    max_suggestion: Option<u8>,

    #[arg(short, long, default_value_t = false)]
    backup: bool,

    #[arg(short, long, requires("backup"))]
    output: Option<String>,
}

pub trait AppBuilder {
    fn build() -> Pin<Box<dyn core::future::Future<Output = Result<Arbor, Box<dyn Error>>> + Send>>
    where
        Self: Sync,
    {
        Box::pin(async move {
            let args = Args::parse();

            let output = if let Some(ref o) = args.output {
                Some(o.as_str())
            } else {
                None
            };

            Ok(Arbor {
                autocomplete: Autocomplete::build(
                    args.language.clone(),
                    args.thread_count,
                    args.max_suggestion,
                    args.backup,
                    output,
                )
                .await?,
            })
        })
    }
}

pub struct Arbor {
    pub autocomplete: Autocomplete,
}

impl AppBuilder for Arbor {}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn it_initializes_args() -> Result<(), Box<dyn Error>> {
        let args = get_args(&["arbor", "--language", "en-US"])?;

        assert_eq!(args.language, Some("en-US".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn it_builds_app() -> Result<(), Box<dyn Error>> {
        let mut arbor = Arbor::build().await?;
        let word = "hello".to_string();

        arbor.autocomplete.insert_word(word.clone()).await?;

        let suggestion = arbor.autocomplete.suggest_word("hel").await?;

        assert_eq!(suggestion.iter().nth(0).unwrap().to_owned(), word);

        Ok(())
    }

    fn get_args<I, T>(itr: I) -> Result<Args, Box<dyn Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        return Ok(Args::try_parse_from(itr)?);
    }
}
