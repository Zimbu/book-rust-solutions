use clap::Parser;
use anyhow::Result;
use core::str;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(default_values = ["-"], value_name="FILE")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'), 
        long, 
        default_value = "10", 
        value_name = "LINES",
        group = "numbering",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes
    #[arg(
        short('c'), 
        long, 
        value_name = "BYTES",
        group = "numbering",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut is_first = true;
    for filename in &args.files {
        if args.files.len() > 1 {
            if !is_first {
                println!();
            }
            println!("==> {filename} <==");
            is_first = false;
        }

        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(mut reader) => {
                match args.bytes {
                    None => {
                      let mut line_count = 0;
                      let mut line = String::new();

                      let mut bytes_read = reader.read_line(&mut line)?;
                      while (line_count < args.lines) && (bytes_read > 0) {
                        print!("{line}");
                        line.clear();
                        line_count += 1;
                        bytes_read = reader.read_line(&mut line)?;
                      }
                    },
                    Some(byte_count) => {
                      let bytes: Result<Vec<_>, _> = reader.bytes().take(byte_count as usize).collect();
                      print!("{}", String::from_utf8_lossy(&bytes?));
                    },
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
