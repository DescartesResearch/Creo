use creo::cli;
use creo::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: cli::Args = argh::from_env();
    let mut stdout = std::io::stdout().lock();
    creo::entrypoint(args, &mut stdout).await
}
