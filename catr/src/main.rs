use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug,Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// Input file(s)
    #[arg(default_values = ["-"])]
    files: Vec<String>,

    /// Number lines
    #[arg(short('n'), long("number"), group = "numbering")]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"), group = "numbering")]
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> { 
    for  filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(reader) => {
                let mut blank_lines_count = 0;
                for (line_num, line_result) in reader.lines().enumerate() {
                    let line = line_result?;
                    if args.number_nonblank_lines && line.is_empty() {
                        blank_lines_count += 1;
                        println!();
                    } else if args.number_lines || (args.number_nonblank_lines && !line.is_empty()) {
                      println!("{:>6}\t{line}", line_num + 1 - blank_lines_count);
                    } else {
                      println!("{line}");
                    }
                }
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
