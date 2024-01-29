/*
A Command-line tool to work with OpenAI's API to complete sentences
 */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A Command-line tool to work with OpenAI's API to summarize text"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Complete {
        #[clap(short, long)]
        text: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Complete { text }) => {
            let result = rsummarize::summarize(&text).await?;
            println!("{}", result);
        }
        None => {
            println!("No command provided");
        }
    }
    Ok(())
}
