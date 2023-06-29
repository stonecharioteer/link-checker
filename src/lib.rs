//! This library provides functions to check all files in a directory for URLs,
//! and checks if these URLs are working.
#![allow(unused)]
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
use std::path::PathBuf;

use thiserror::Error;
use url::Url;

/// custom errors for this crate
#[derive(Error, Debug)]
enum LinkCheckerErrors {
    #[error("Raise when there is an invalid URL")]
    InvalidUrl,

    #[error("Network unavailable.")]
    NetworkUnavailable,

    #[error("URL Unreachable")]
    UrlUnreachable,

    #[error("Unknown")]
    Unknown,
}

/// Given a directory, returns all files within it.
pub fn find_files(directory: &PathBuf, respect_gitignore: &bool) -> Vec<PathBuf> {
    todo!()
}
/// given a string, finds all URLs within it.
fn find_urls(s: String) -> Result<Vec<Url>, LinkCheckerErrors> {
    todo!();
}

/// given a URL, sends it a GET request and then checks if the URL responds with a 200 or not.
fn validate_url(url: Url) -> Result<bool, LinkCheckerErrors> {
    todo!()
}

#[cfg(test)]
mod tests {

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
