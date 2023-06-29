use clap::Parser;
use link_checker::find_files;
use std::env;
use std::fs::write;
use std::path::PathBuf;
use std::process::exit;

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
    let _github_output_path = env::var("GITHUB_OUTPUT");

    let args = Args::parse();
    let working_directory = if let Some(d) = args.directory {
        d
    } else {
        env::current_dir().unwrap()
    };
    let respect_gitignore = args.respect_gitignore;

    let file_list: Vec<PathBuf> = find_files(&working_directory, &respect_gitignore);

    println!("{working_directory:?}, {respect_gitignore}");

    // if !error.is_empty() {
    //     eprintln!("Error: {error}");
    //     write(github_output_path, format!("error={error}")).unwrap();
    //     exit(1);
    // }
}
