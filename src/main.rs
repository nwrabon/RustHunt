use clap::Parser;
use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::process;

// ======================== TODO IF WE CAN GET TO IT ========================================
// *Allow filtering of output by response codes via passed args - Mark Elkins can complete this too (Got done 200, 404, 406, 403, and all)
// *Colorize output i.e green status codes for 200 ok, red for 404 not found - Mark Elkins will complete this (200, 404, 406, 403 done)
// Actually make it use async for speed - Chase is currently doing this
// *Cool logo to display via ASCII art on startup
// Filter output while requests are running <- fancy hard async stuff I doubt we do but would be awesome
// Recurse argument to bust directories found by the current bust
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

    // filter for output
    #[arg(short, long)]
    filter: String,
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
async fn make_request(full_path: &String, filter: &String) {
    let _res = match reqwest::get(full_path).await {
        Ok(res) => {
            if (filter.eq("200") && res.status() == 200) || (filter.eq("a") && res.status() == 200)  {
                    println!("\r[{}] - {}", res.status().as_str().green(), full_path); 
            } else if (filter.eq("404") && res.status() == 404) || (filter.eq("a") && res.status() == 404)  {
                println!("\r[{}] - {}", res.status().as_str().red(), full_path); 
            } else if (filter.eq("406") && res.status() == 406) || (filter.eq("a") && res.status() == 406)  {
                println!("\r[{}] - {}", res.status().as_str().red(), full_path); 
            } else if (filter.eq("403") && res.status() == 403) || (filter.eq("a") && res.status() == 403)  {
                println!("\r[{}] - {}", res.status().as_str().yellow(), full_path); 
            } else if filter.eq("a") {
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

    // Displays loading animation
    let mut sp = Spinner::new(Spinners::Triangle, "Searching...".into());

    //create a vector of concurrent task handlers
    let mut task_handlers = vec![];

    let wordlist = File::open(args.wordlist).unwrap();
    let reader = BufReader::new(wordlist);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let full_path = format!("{}/{}", args.url, line);
        let filter = format!("{}", args.filter);

        let handler = tokio::spawn(async move {
            make_request(&full_path, &filter).await;
        });
        task_handlers.push(handler);
    }

    for handler in task_handlers {
        handler.await.unwrap();
    }

    sp.stop_and_persist("✅", " Finished search!".green().to_string());
}
