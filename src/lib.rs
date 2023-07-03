//! This library provides functions to check all files in a directory for URLs,
//! and checks if these URLs are working.
#![allow(unused)]
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
use std::{
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;
use url::Url;
use walkdir::WalkDir;

/// Result Struct
#[derive(Hash, Debug, Clone)]
pub struct UrlResult {
    /// list of files that this URL appears in
    pub file_list: Vec<PathBuf>,
    /// the URL query result
    pub result: Result<bool, LinkCheckerErrors>,
}

impl UrlResult {
    /// constructor
    pub fn new(f: PathBuf, result: Result<bool, LinkCheckerErrors>) -> Self {
        Self {
            file_list: vec![f],
            result,
        }
    }
    /// adds a file to the file list
    pub fn add_file(&mut self, f: PathBuf) {
        self.file_list.push(f)
    }

    /// returns the error if it exists
    pub fn error(&self) -> Option<LinkCheckerErrors> {
        match &self.result {
            Err(e) => Some(e.clone()),
            _ => None,
        }
    }
}
/// custom errors for this crate
#[derive(Error, Debug, Hash, Clone)]
pub enum LinkCheckerErrors {
    /// the URL format is invalid
    #[error("Raise when there is an invalid URL")]
    InvalidUrl,

    /// this computer cannot reach the network
    #[error("Network unavailable.")]
    NetworkUnavailable,

    /// The URL is unreachable
    #[error("URL Unreachable")]
    UrlUnreachable,

    /// I don't know what went wrong
    #[error("Unknown")]
    Unknown,
}

/// Given a directory, returns all files within it.
pub fn find_files(directory: &Path, respect_gitignore: bool) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = WalkDir::new(directory)
        .into_iter()
        .map(|x| x.unwrap().path().to_owned())
        .collect();
    if respect_gitignore {
        // NOTE: Only supporting root-level .gitignore files now.
        let gitignore_file = Path::new("/.gitignore");
        let gitignore: Option<String> = if gitignore_file.is_file() {
            Some(fs::read_to_string(gitignore_file).unwrap())
        } else {
            // WARN about no gitignore.
            None
        };

        match gitignore {
            Some(gitignore_rules) => {
                let rules = gitignore_rules.lines();
                todo!()
            }
            None => todo!("warn about no gitignore"),
        };
    };
    // remove directories from this.
    file_list = file_list.into_iter().filter(|x| x.is_file()).collect();
    file_list
}
/// given a string, finds all URLs within it.
pub fn find_urls(s: String) -> Result<Vec<Url>, LinkCheckerErrors> {
    todo!();
}

/// given a URL, sends it a GET request and then checks if the URL responds with a 200 or not.
pub fn validate_url(url: &Url) -> Result<bool, LinkCheckerErrors> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// tests that given a folder path, the `find_files` function
    /// can retrieve all the files in it.
    #[test]
    fn test_find_files() {
        let dir = Path::new("./");
        find_files(dir, false);
    }

    /// tests that given a string containing a URL among other things, the `get_url` function
    /// can construct a URL from it.
    #[test]
    fn test_url_regex() {
        todo!()
    }

    /// tests that given a markdown-compliant string containing many URLs, the `find_urls` function
    /// can return a vector of urls.
    #[test]
    fn test_markdown_url_finder() {
        todo!()
    }

    /// tests that given a Rust source code file containing many URLs, the `find_urls` function
    /// can return a vector of urls
    #[test]
    fn test_rust_url_finder() {
        todo!()
    }
}
