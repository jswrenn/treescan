use walkdir::*;
use std::path::PathBuf;
use structopt::StructOpt;
use regex::Regex;
use std::path::Path;


#[derive(Debug, StructOpt)]
#[structopt(name = "treescan", about = "Find lines matching a pattern.")]
struct Opt {
    /// Print lines that match this pattern.
    pattern: String,
    /// Recursively search files in this path.
    root: PathBuf,
}


fn search(pattern: &Regex, path: &Path) {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    for (i, line) in lines.enumerate() {
      if let Ok(line) = line {
        if pattern.is_match(&line) {
            println!("{}:{}\t{}", path.to_string_lossy(), i, line);
        }
      } else {
        /* TODO: print a nice error message? */
      }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
    let arguments = Opt::from_args();

    let pattern = Regex::new(&arguments.pattern).unwrap();

    let walker = WalkDir::new(arguments.root).into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e)).flatten() {
      let metadata = entry.metadata().unwrap();
      if metadata.is_file() {
          search(&pattern, entry.path())
      }
    }
}
