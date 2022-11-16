use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use spinners::{Spinner, Spinners};
use std::process;
use colored::Colorize;

// ======================== TODO IF WE CAN GET TO IT ========================================
// *Allow filtering of output by response codes via passed args - Mark Elkins can complete this too
// *Colorize output i.e green status codes for 200 ok, red for 404 not found - Mark Elkins will complete this (200 and 404 done)
// Actually make it use async for speed
// *Cool logo to display via ASCII art on startup
// Filter output while requests are running <- fancy hard async stuff I doubt we do but would be awesome
// Recurse argument to bust directories find by the current bust
//
// * = We should make it a goal to finish / not too hard

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

/// Makes a request to the given full_path
/// Outputs the status code of the request or
/// Outputs Error if request failed
async fn make_request(full_path: &String) {    
    let _res = match reqwest::get(full_path).await{
        Ok(res)=>{
            if res.status() ==  200 {
                println!("\r[{}] - {}", res.status().as_str().green(), full_path);
            }
            else if res.status() == 404 || res.status() == 406 {
                println!("\r[{}] - {}", res.status().as_str().red(), full_path);
            }
            else if res.status() == 403 {
                println!("\r[{}] - {}", res.status().as_str().yellow(), full_path);
            }   
            else{
                println!("\r[{}] - {}", res.status(), full_path);
            }
        }
        Err(_e)=>{
            println!("\r{}", "[Error] Make sure given url exists".red());
            process::exit(0);
        }
    };
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Displays loading animation
    let mut sp = Spinner::new(Spinners::Triangle,"Searching...".into());

    let wordlist = File::open(args.wordlist).unwrap();
    let reader = BufReader::new(wordlist);
    for (_index, line) in reader.lines().enumerate(){
        let line = line.unwrap();
        let full_path = format!("{}/{}", args.url, line);

        make_request(&full_path).await;
    }

    sp.stop_and_persist("✅", " Finished search!".green().to_string());
}
