use clap::Parser;
use color_eyre::eyre::Result;
use regex::Regex;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    include: String,
    #[arg(short, long, default_value = "")]
    exclude: String,
    #[arg(short, long, default_value = "")]
    pattern: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let include = args.include;
    let exclude = args.exclude;
    let pattern = Regex::new(&args.pattern)?;
    let re = Regex::new(r"^[a-z]{5}$")?;
    let word_list = include_str!("./dutch").lines();

    word_list.for_each(|word| {
        if !re.is_match(word) || !pattern.is_match(word) {
            return;
        }
        let include_chars = include.chars();
        for c in include_chars {
            if !word.contains(c) {
                return;
            }
        }

        let exclude_chars = exclude.chars();
        for c in exclude_chars {
            if word.contains(c) {
                return;
            }
        }

        println!("{}", word);
    });
    Ok(())
}
