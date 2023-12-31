#![allow(unused)]
use clap::Parser;
use link_checker::{find_files, find_urls, validate_url, UrlResult};
use std::collections::HashMap;
use std::env;
use std::fs::write;
use std::path::PathBuf;
use std::process::exit;
use tracing_subscriber::EnvFilter;
use url::Url;

use tracing::{debug, info, trace, warn};

/// Program to check all files in a directory
/// for unreachable URLs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory.
    /// Default is current working directory.
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// Respect gitignore.
    /// Default is true
    #[arg(short, long, default_value_t = true)]
    respect_gitignore: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("LINK_CHECKER_LOG"))
        .init();

    let github_output_path = env::var("GITHUB_OUTPUT");
    debug!(?github_output_path);

    let args = Args::parse();
    let working_directory = if let Some(d) = args.directory {
        d
    } else {
        env::current_dir().unwrap()
    };
    let respect_gitignore = args.respect_gitignore;
    debug!(?working_directory);
    debug!(respect_gitignore);

    let file_list: Vec<PathBuf> = find_files(&working_directory, respect_gitignore);
    debug!("Files found: {}", file_list.len());

    let mut url_results: HashMap<Url, UrlResult> = HashMap::new();
    for file in file_list {
        // read the file
        let s: String = "".to_owned();
        // check each file
        let urls = find_urls(s).unwrap();
        // store url result in a hashmap that contains the list of files
        // it appears in, and the result.
        for url in urls {
            // check if the url has been validated.
            if !(url_results.contains_key(&url)) {
                let result = validate_url(&url);
                let url_result = UrlResult::new(file.clone(), result);
                url_results.insert(url.clone(), url_result);
            } else {
                url_results.get_mut(&url).unwrap().add_file(file.clone());
            }
        }
    }

    let mut error = false;
    let write_to_github = matches!(github_output_path, Ok(_));
    for (link, result) in url_results {
        if let Some(e) = result.error() {
            if !error {
                error = true;
                if !write_to_github {
                    eprintln!("Not writing to github error path since the envar is not set. Running locally perhaps?");
                }
            };
            let err = format!(
                "Failed to get: `{}`. Error: {e:?}. Appears in {} files",
                link,
                result.file_list.len()
            );
            eprintln!("{err}");
            if write_to_github {
                write(github_output_path.clone().unwrap(), format!("error={err}")).unwrap();
            }
        }
    }

    if error {
        exit(1);
    }
}
