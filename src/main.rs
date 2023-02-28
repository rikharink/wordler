use color_eyre::eyre::Result;
use std::{fs::File, io::{self, BufRead}, path::Path};
use regex::Regex;
use clap::Parser;

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let include = args.include;
    let exclude = args.exclude;
    let pattern = Regex::new(&args.pattern)?;
    let re = Regex::new(r"^[a-z]{5}$")?;
    if let Ok(word_list) = read_lines("/usr/share/dict/dutch") {
        word_list.for_each(|line| {
            if let Ok(word) = line {
                if !re.is_match(&word) || !pattern.is_match(&word) {
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
            }
        });
    }

    Ok(())
}
