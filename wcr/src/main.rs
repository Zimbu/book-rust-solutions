use anyhow::Result;
use clap::Parser;
use core::str;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `wc`
struct Args {
    /// Input file(s)
    #[arg(default_values = ["-"], value_name="FILE")]
    files: Vec<String>,

    /// Show line count
    #[arg(short('l'), long, default_value = "false", value_name = "LINES")]
    lines: bool,

    /// Show word count
    #[arg(short('w'), long, default_value = "false", value_name = "LINES")]
    words: bool,

    /// Show byte count
    #[arg(
        short('c'),
        long,
        default_value = "false",
        value_name = "LINES",
        group = "numbering"
    )]
    bytes: bool,

    /// Show character count
    #[arg(
        short('m'),
        long,
        default_value = "false",
        value_name = "LINES",
        group = "numbering"
    )]
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> Result<()> {
    if !args.lines && !args.words && !args.chars && !args.bytes {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                let counts = count(file)?;
                total_lines += counts.num_lines;
                total_words += counts.num_words;
                total_bytes += counts.num_bytes;
                total_chars += counts.num_chars;

                if args.lines {
                    print!("{0:>8}", counts.num_lines);
                }
                if args.words {
                    print!("{0:>8}", counts.num_words);
                }
                if args.bytes {
                    print!("{0:>8}", counts.num_bytes);
                }
                if args.chars {
                    print!("{0:>8}", counts.num_chars);
                }
                if "-" != filename {
                    println!(" {filename}");
                } else {
                    println!();
                }
            }
        }
    }
    if args.files.len() > 1 {
        if args.lines {
            print!("{0:>8}", total_lines);
        }
        if args.words {
            print!("{0:>8}", total_words);
        }
        if args.bytes {
            print!("{0:>8}", total_bytes);
        }
        if args.chars {
            print!("{0:>8}", total_chars);
        }
        println!(" total");
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buf = String::new();
    loop {
        let line_num_bytes = file.read_line(&mut buf)?;
        if line_num_bytes == 0 {
            break;
        }

        num_lines += 1;
        num_bytes += line_num_bytes;
        num_chars += buf.chars().count();
        num_words += buf.split_whitespace().count();

        buf.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        // Arrange
        let text = "I don't want the world.\nI just want your half.\r\n";

        // Act
        let info = count(Cursor::new(text));

        // Assert
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
