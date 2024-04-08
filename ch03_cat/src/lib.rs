use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct Config {
    #[arg(default_value = "-")]
    files: Vec<String>,
    #[arg(short, long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf_read) => {
                let mut empty_count = 0;
                for (index, line) in buf_read.lines().enumerate() {
                    match line {
                        Ok(l) => {
                            let prefix = if config.number_lines {
                                format!("{:6}\t", index + 1)
                            } else if config.number_nonblank_lines {
                                if l.trim().is_empty() {
                                    empty_count += 1;
                                    String::new()
                                } else {
                                    format!("{:6}\t", index + 1 - empty_count)
                                }
                            } else {
                                String::new()
                            };
                            println!("{}{}", prefix, l)
                        }
                        Err(err) => eprintln!("Failed to read line: {}", err),
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::try_parse();
    config.map_err(|e| e.into())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
