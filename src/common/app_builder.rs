use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "en-US")]
    language: String,

    #[arg(short, long, default_value_t = 2)]
    thread_count: u8,

    #[arg(short, long, default_value_t = 10)]
    max_suggestion: u8,

    #[arg(short, long, default_value_t = false)]
    backup: bool,

    #[arg(short, long, requires("backup"))]
    output: Option<String>,
}

pub trait AppBuilder {
    fn get_args(&self) -> &Args;

    fn build(&self) -> Result<(), Box<dyn Error>> {
        let _args = self.get_args();

        Ok(())
    }
}

impl AppBuilder for Args {
    fn get_args(&self) -> &Args {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn it_initializes_args() -> Result<(), Box<dyn Error>> {
        let args = get_args(&["arbor"])?;

        assert_eq!(args.language, "en-US");

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
