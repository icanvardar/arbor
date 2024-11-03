use std::error::Error;

use arbor::util::repl::Repl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Repl::new().await?.run().await?;

    Ok(())
}
