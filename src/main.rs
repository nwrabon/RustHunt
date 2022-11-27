use clap::ArgGroup;
use clap::Parser;
use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::process;

// ======================== TODO IF WE CAN GET TO IT ========================================
// *Allow filtering of output by response codes via passed args - Mark Elkins can complete this too
// *Colorize output i.e green status codes for 200 ok, red for 404 not found - Mark Elkins will complete this (200 and 404 done)
// Actually make it use async for speed - Done
// *Cool logo to display via ASCII art on startup - Done
// Filter output while requests are running <- fancy hard async stuff I doubt we do but would be awesome
// Recurse argument to bust directories found by the current bust
//
// * = We should make it a goal to finish / not too hard

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("filter")
        .required(false)
        .args(["include", "exclude"]),
    ))]
struct Args {
    /// **Required** url of site to bust
    #[arg(short, long)]
    url: String,

    /// **Required** path to wordlist of directories and files to try
    #[arg(short, long)]
    wordlist: PathBuf,

    /// list of status codes to exclude delimited by ':'
    #[arg(
        short,
        long,
        value_parser,
        use_value_delimiter = true,
        value_delimiter = ':'
    )]
    exclude: Option<Vec<i32>>,

    /// list of status codes to include delimited by ':'
    #[arg(
        short,
        long,
        value_parser,
        use_value_delimiter = true,
        value_delimiter = ':'
    )]
    include: Option<Vec<i32>>,
}

fn output_art() {
    let art_path = PathBuf::from("./crosshair.txt");
    let mut art_file = File::open(art_path).unwrap();
    let mut art = String::new();
    art_file.read_to_string(&mut art).unwrap();

    println!("{}\n", art.truecolor(245, 102, 0));
}

/// Makes a request to the given full_path
/// Outputs the status code of the request or
/// Outputs Error if request failed
async fn make_request(full_path: &String, include_list: &mut Vec<i32>) {
    let _res = match reqwest::get(full_path).await {
        Ok(res) => {
            if res.status() == 200 {
                println!("\r[{}] - {}", res.status().as_str().green(), full_path);
            } else if res.status() == 404 || res.status() == 406 {
                println!("\r[{}] - {}", res.status().as_str().red(), full_path);
            } else if res.status() == 403 {
                println!("\r[{}] - {}", res.status().as_str().yellow(), full_path);
            } else {
                println!("\r[{}] - {}", res.status(), full_path);
            }
        }
        Err(_e) => {
            println!("\r{}", "[Error] Make sure given url exists".red());
            process::exit(0);
        }
    };
}

#[tokio::main]
async fn main() {
    output_art();
    let args = Args::parse();

    let mut include_list = args.include.unwrap_or_else(|| Vec::new());

    // Displays loading animation
    let mut sp = Spinner::new(Spinners::Triangle, "Searching...".into());

    //create a vector of concurrent task handlers
    let mut task_handlers = vec![];

    let wordlist = File::open(args.wordlist).unwrap();
    let reader = BufReader::new(wordlist);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let full_path = format!("{}/{}", args.url, line);

        let handler = tokio::spawn(async move {
            make_request(&full_path, &mut include_list).await;
        });
        task_handlers.push(handler);
    }

    for handler in task_handlers {
        handler.await.unwrap();
    }

    sp.stop_and_persist("✅", " Finished search!".green().to_string());
}
