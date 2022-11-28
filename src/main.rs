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

    /// list of file extensions codes to exclude delimited by ':'
    #[arg(long, value_parser, use_value_delimiter = true, value_delimiter = ':')]
    extensions: Option<Vec<String>>,
}

fn output_art() {
    let art_path = PathBuf::from("./crosshair.txt");
    let mut art_file = File::open(art_path).unwrap();
    let mut art = String::new();
    art_file.read_to_string(&mut art).unwrap();

    println!("{}\n", art.truecolor(245, 102, 0));
}

fn print_response(status: u16, path: String) {
    println!("\r[{}] - {}", status, path);
}

/// Makes a request to the given request_path
/// Returns the status code of the request or
/// Outputs Error if request failed
async fn make_request(request_path: &String) -> (u16, &String) {
    let _res = match reqwest::get(request_path).await {
        Ok(res) => {
            return (res.status().as_u16(), &request_path);
        }
        Err(_e) => {
            println!("\r{}", "[Error] Make sure given url exists".red());
            process::exit(0);
        }
    };
}

/// Sends base request, iff the directory is found, we bust it as new base_path
/// Sends all remaining requests by calling all file extensions on the base word
async fn bust_url(
    base_path: &String,
    word_list: &Vec<String>,
    file_extensions: &Vec<String>,
) -> (u16, String) {
}

#[tokio::main]
async fn main() {
    output_art();
    let args = Args::parse();

    // Displays loading animation
    let mut sp = Spinner::new(Spinners::Triangle, "Searching...".into());

    // Create an empty vector to hold the words passed in by word list
    let mut word_list: Vec<String> = vec![];

    // populate the vector with the given wordlist
    let word_file = File::open(args.wordlist).unwrap();
    let reader = BufReader::new(word_file);
    for (_index, line) in reader.lines().enumerate() {
        word_list.append(line);
    }

    let file_extensions: Vec<String> = args.extensions.clone().unwrap_or_else(|| {
        vec![
            String::from("AIFF"),
            String::from("AIF"),
            String::from("AU"),
            String::from("AVI"),
            String::from("BAT"),
            String::from("BMP"),
            String::from("CLASS"),
            String::from("JAVA"),
            String::from("CSV"),
            String::from("CVS"),
            String::from("DBF"),
            String::from("DIF"),
            String::from("DOC"),
            String::from("DOCX"),
            String::from("EPS"),
            String::from("EXE"),
            String::from("FM3"),
            String::from("GIF"),
            String::from("HQX"),
            String::from("HTM"),
            String::from("HTML"),
            String::from("JPG"),
            String::from("JPEG"),
            String::from("MAC"),
            String::from("MAP"),
            String::from("MDB"),
            String::from("MID"),
            String::from("MIDI"),
            String::from("MOV"),
            String::from("QT"),
            String::from("MTB"),
            String::from("MTW"),
            String::from("PDF"),
            String::from("P65"),
            String::from("T65"),
            String::from("PNG"),
            String::from("PPT"),
            String::from("PPTX"),
            String::from("PSD"),
            String::from("PSP"),
            String::from("QXD"),
            String::from("RA"),
            String::from("RTF"),
            String::from("SIT"),
            String::from("TAR"),
            String::from("TIF"),
            String::from("TXT"),
            String::from("WAV"),
            String::from("WK3"),
            String::from("WKS"),
            String::from("WPD"),
            String::from("WP5"),
            String::from("XLSX"),
            String::from("XLSX"),
            String::from("aiff"),
            String::from("aif"),
            String::from("au"),
            String::from("avi"),
            String::from("bat"),
            String::from("bmp"),
            String::from("class"),
            String::from("java"),
            String::from("csv"),
            String::from("cvs"),
            String::from("dbf"),
            String::from("dif"),
            String::from("doc"),
            String::from("docx"),
            String::from("eps"),
            String::from("exe"),
            String::from("fm3"),
            String::from("gif"),
            String::from("hqx"),
            String::from("htm"),
            String::from("html"),
            String::from("jpg"),
            String::from("jpeg"),
            String::from("mac"),
            String::from("map"),
            String::from("mdb"),
            String::from("mid"),
            String::from("midi"),
            String::from("mov"),
            String::from("qt"),
            String::from("mtb"),
            String::from("mtw"),
            String::from("pdf"),
            String::from("p65"),
            String::from("t65"),
            String::from("png"),
            String::from("ppt"),
            String::from("pptx"),
            String::from("psd"),
            String::from("psp"),
            String::from("qxd"),
            String::from("ra"),
            String::from("rtf"),
            String::from("sit"),
            String::from("tar"),
            String::from("tif"),
            String::from("txt"),
            String::from("wav"),
            String::from("wk3"),
            String::from("wks"),
            String::from("wpd"),
            String::from("wp5"),
            String::from("xlsx"),
            String::from("xlsx"),
        ]
    });

    //create a vector of concurrent task handlers
    let mut task_handlers = vec![];

    for word in word_list {
        let base_path = args.url;
        let include_list = args.include.clone().unwrap_or_else(|| Vec::new());
        let exclude_list = args.exclude.clone().unwrap_or_else(|| Vec::new());

        let handler = tokio::spawn(async move {
            bust_url(&base_path, &include_list, &exclude_list).await;
        });
        task_handlers.push(handler);
    }

    for handler in task_handlers {
        handler.await.unwrap();
    }

    sp.stop_and_persist("✅", " Finished search!".green().to_string());
}
