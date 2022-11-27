use clap::ArgGroup;
use clap::Parser;
use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::process;

// ======================== TODO IF WE CAN GET TO IT ========================================
// Recurse argument to bust directories found by the current bust
// check for default file extensions (in a file)
// options to specify file extensions
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
    exclude: Option<Vec<u16>>,

    /// list of status codes to include delimited by ':'
    #[arg(
        short,
        long,
        value_parser,
        use_value_delimiter = true,
        value_delimiter = ':'
    )]
    include: Option<Vec<u16>>,
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
async fn make_request(full_path: &String, include_list: &Vec<u16>, exclude_list: &Vec<u16>) {
    let _res = match reqwest::get(full_path).await {
        Ok(res) => {
            if include_list.contains(&res.status().as_u16())
                || (!exclude_list.is_empty() && !exclude_list.contains(&res.status().as_u16())
                    || (exclude_list.is_empty() && include_list.is_empty()))
            {
                if res.status() == 200 {
                    println!("\r[{}] - {}", res.status().as_str().green(), full_path);
                } else if res.status() == 404 || res.status() == 406 || res.status() == 400 {
                    println!("\r[{}] - {}", res.status().as_str().red(), full_path);
                } else if res.status() == 403 || res.status() == 429 || res.status() == 451 {
                    println!("\r[{}] - {}", res.status().as_str().yellow(), full_path);
                } else if res.status() == 500 || res.status() == 502 || res.status() == 503 {
                    println!("\r[{}] - {}", res.status().as_str().purple(), full_path);
                } else {
                    println!("\r[{}] - {}", res.status(), full_path);
                }
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

    // Displays loading animation
    let mut sp = Spinner::new(Spinners::Triangle, "Searching...".into());

    //create a vector of concurrent task handlers
    let mut task_handlers = vec![];

    let wordlist = File::open(args.wordlist).unwrap();
    let reader = BufReader::new(wordlist);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let full_path = format!("{}/{}", args.url, line);
        let include_list = args.include.clone().unwrap_or_else(|| Vec::new());
        let exclude_list = args.exclude.clone().unwrap_or_else(|| Vec::new());

        let handler = tokio::spawn(async move {
            make_request(&full_path, &include_list, &exclude_list).await;
        });
        task_handlers.push(handler);
    }

    for handler in task_handlers {
        handler.await.unwrap();
    }

    sp.stop_and_persist("✅", " Finished search!".green().to_string());
}
