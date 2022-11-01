use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// url of site to bust
   #[arg(short, long)]
   url: String,

   /// path to wordlist of directories and files to try
   #[arg(short, long)]
   wordlist: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{}", args.url);
    println!("{}", args.wordlist.display());
}
