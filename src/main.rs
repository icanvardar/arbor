use std::error::Error;

use arbor::common::app_builder::{AppBuilder, Arbor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Arbor::build().await?;

    Ok(())
}
