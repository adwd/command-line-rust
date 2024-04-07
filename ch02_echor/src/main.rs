use clap::Parser;

/// An Example CLI Command
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Cli {
    /// Input text
    text: Vec<String>,

    /// Do not print newline
    #[arg(short('n'), long)]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    // println!("{:#?}", cli);

    let text = cli.text;
    let omit_newline = cli.omit_newline;

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
