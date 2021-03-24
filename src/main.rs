use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(name = "treescan", about = "Find lines matching a pattern.")]
struct Opt {
    /// Print lines that match this pattern.
    pattern: Regex,
    /// Recursively search files in this path.
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Opt { pattern, path } = Opt::from_args();

    let walker = WalkDir::new(path).into_iter();

    for entry in walker.flatten() {
        let is_file = entry.metadata().map(|m| m.is_file()).unwrap_or(false);

        if !is_file {
            continue;
        }

        let path = entry.path();

        if let Ok(file) = File::open(path) {
            let lines = BufReader::new(file).lines();

            for (i, line) in lines.enumerate() {
                if let Ok(line) = line {
                    if pattern.is_match(&line) {
                        println!("{}:{}\t{}", path.display(), i, line);
                    }
                }
            }
        }
    }

    Ok(())
}
