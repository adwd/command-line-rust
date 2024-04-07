use clap::Parser;
use std::error::Error;
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
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::try_parse();
    config.map_err(|e| e.into())
}
