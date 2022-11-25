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

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// url of site to bust
    #[arg(short, long)]
    url: String,

    /// path to wordlist of directories and files to try
    #[arg(short, long)]
    wordlist: PathBuf,
    
    // exclude filter for output
    #[arg(default_value="none", short, long)]
    exclude_filter: String,

    // include filter for output
    #[arg(default_value="all", short, long)]
    include_filter: String,
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
async fn make_request(full_path: &String, include_filter: &String, exclude_filter: &String) {
    let _res = match reqwest::get(full_path).await {
        Ok(res) => {
            if (include_filter.eq("200") && res.status() == 200) || (include_filter.eq("all") && res.status() == 200) && !exclude_filter.eq("200")  {
                    println!("\r[{}] - {}", res.status().as_str().green(), full_path); 
            } else if (include_filter.eq("404") && res.status() == 404) || (include_filter.eq("all") && res.status() == 404) && !exclude_filter.eq("404") {
                println!("\r[{}] - {}", res.status().as_str().red(), full_path); 
            } else if (include_filter.eq("406") && res.status() == 406) || (include_filter.eq("all") && res.status() == 406) && !exclude_filter.eq("406") {
                println!("\r[{}] - {}", res.status().as_str().red(), full_path); 
            } else if (include_filter.eq("403") && res.status() == 403) || (include_filter.eq("all") && res.status() == 403) && !exclude_filter.eq("403") {
                println!("\r[{}] - {}", res.status().as_str().yellow(), full_path); 
            } else if include_filter.eq("all") && !exclude_filter.eq("all") && exclude_filter.parse::<u16>().unwrap() != res.status() {
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
        let include_filter = format!("{}", args.include_filter);
        let exclude_filter = format!("{}", args.exclude_filter);
        if !include_filter.eq("all") && !exclude_filter.eq("none") {
            println!("\r{}", "[Error] Only 1 filter option allowed".red());
            process::exit(0);
        }

        let handler = tokio::spawn(async move {
            make_request(&full_path, &include_filter, &exclude_filter).await;
        });
        task_handlers.push(handler);
    }

    for handler in task_handlers {
        handler.await.unwrap();
    }

    sp.stop_and_persist("✅", " Finished search!".green().to_string());
}
