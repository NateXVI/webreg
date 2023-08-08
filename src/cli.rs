use std::{
    io::{self, Read},
    path::PathBuf,
};

use clap::Parser;
use libc::isatty;

#[derive(Debug, Parser)]
#[command(name = "Keyword Finder")]
#[command(about = "Test if a list of websites match a given regex")]
#[command(version)]
pub struct Cli {
    /// Comma separated list of urls
    #[arg(short, long, value_delimiter = ',', group = "input")]
    pub urls: Option<Vec<String>>,

    /// A file containing a list of urls
    #[arg(short = 'i', long, group = "input")]
    pub file: Option<PathBuf>,

    /// Case insensitive search
    #[arg(short, long)]
    pub case_insensitive: bool,

    /// Fix urls that don't start with http:// or https://
    #[arg(short, long)]
    pub fix_urls: bool,

    /// Retry failed urls
    #[arg(short, long)]
    pub retry: bool,

    /// Saves the output to the results folder (./results/<regex>)
    #[arg(short, long)]
    pub save: bool,

    /// A regular expression to match against the site content
    pub regex: String,
}

pub fn parse() -> (Cli, Vec<String>) {
    let args = Cli::parse();
    let urls = get_urls(&args);
    let urls = urls
        .iter()
        .map(|url| url.trim().to_string())
        .filter(|url| !url.is_empty())
        .collect::<Vec<String>>();

    let urls = if args.fix_urls {
        let mut fixed_urls = Vec::new();
        for url in urls {
            fixed_urls.push(format_url(&url));
        }
        fixed_urls
    } else {
        urls
    };

    (args, urls)
}

fn get_urls(args: &Cli) -> Vec<String> {
    if let Some(urls) = &args.urls {
        return urls.clone();
    }

    if let Some(file) = &args.file {
        let contents = std::fs::read_to_string(file).expect("Failed to read file");

        return contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>();
    }

    let is_interactive = unsafe { isatty(libc::STDIN_FILENO) == 1 };

    if !is_interactive {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();

        return buffer
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>();
    }

    std::process::exit(1);
}

fn format_url(url: &str) -> String {
    let re = regex::Regex::new(r"^https?://").unwrap();
    if re.is_match(url) {
        return url.to_string();
    } else {
        return format!("http://{url}");
    }
}
