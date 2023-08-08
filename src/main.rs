mod cli;
mod fetch;

use indicatif::{ProgressBar, ProgressStyle};
use regex;
use rocksdb::DB;
use std::{
    fs,
    sync::{mpsc::channel, Arc, Mutex},
    time::Duration,
};
use threadpool::ThreadPool;

use crate::cli::parse;

enum Result {
    Match,
    NoMatch,
    Error,
}

fn main() {
    let (args, urls) = parse();

    let regex = args.regex;

    let db = Arc::new(DB::open_default("tmp/cache").unwrap());
    let pool = ThreadPool::default();
    let (tx, rx) = channel();

    let total = urls.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            // .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta}) {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    let pb = Arc::new(Mutex::new(pb));

    for url in &urls {
        let db = db.clone();
        let tx = tx.clone();
        let url = url.clone();
        let c_keyword = regex.clone();
        let pb_clone = pb.clone();

        pool.execute(move || {
            let result = fetch::fetch_url(&url, &db, args.retry);
            let message: String;
            match result {
                Ok(content) => {
                    let re = regex::RegexBuilder::new(&c_keyword)
                        .case_insensitive(args.case_insensitive)
                        .build()
                        .unwrap();
                    if re.is_match(&content) {
                        message = format!("Fetched {url} ✅", url = url);
                        tx.send((Result::Match, url)).unwrap();
                    } else {
                        message = format!("Fetched {url}", url = url);
                        tx.send((Result::NoMatch, url)).unwrap();
                    }
                }
                Err(_) => {
                    message = format!("Error {url} ❌", url = url);
                    tx.send((Result::Error, url)).unwrap();
                }
            }
            pb_clone.lock().unwrap().inc(1);
            pb_clone.lock().unwrap().set_message(message);
        })
    }

    pool.join();

    let mut matches: Vec<String> = Vec::new();
    let mut no_matches: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for _ in 0..urls.len() {
        let (result, url) = rx.recv().unwrap();
        match result {
            Result::Match => matches.push(url),
            Result::NoMatch => no_matches.push(url),
            Result::Error => errors.push(url),
        }
    }

    pb.lock().unwrap().finish_and_clear();

    if args.save {
        save_results(&regex, matches.clone(), no_matches.clone(), errors.clone());
    } else if !matches.is_empty() {
        let output = matches.join("\n");
        print!("{}\n", output);
    }
}

fn save_results(folder: &str, matches: Vec<String>, no_matches: Vec<String>, errors: Vec<String>) {
    fs::create_dir_all(format!("results/{folder}")).unwrap();

    fs::write(format!("results/{folder}/matches.txt"), matches.join("\n")).unwrap();
    fs::write(
        format!("results/{folder}/no_matches.txt"),
        no_matches.join("\n"),
    )
    .unwrap();
    fs::write(format!("results/{folder}/errors.txt"), errors.join("\n")).unwrap();
}
