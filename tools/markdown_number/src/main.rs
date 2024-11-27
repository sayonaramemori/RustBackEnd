use std::fs;
use std::io::{self, BufRead, Write};
use clap::Parser;

/// Add numbers for headline, with suport of withdraw
#[derive(Parser,Debug)]
#[command(about, long_about=None)]
struct Args {
    /// File of the input
    #[arg(short,long)]
    input: String,

    /// File of the output
    #[arg(short,long)]
    output: Option<String>,

    /// Withdraw the numbers
    #[arg(short,long)]
    withdraw: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Input and output file paths
    let input_file = args.input;
    let mut output_file = "output.md".to_string();
    if let Some(name) = args.output.as_ref() {
        if name.ends_with(".md"){
            output_file = name.to_string();
        }else{
            output_file = name.to_string() + ".md";
        }
    }
    if args.withdraw {
        remove_chapter_numbers(&input_file,&output_file)
    }else{
        add_chapter_numbers(&input_file,&output_file)
    }
}

/// Adds chapter numbers to headlines in a Markdown file.
fn add_chapter_numbers(input_file: &str, output_file: &str) -> io::Result<()> {
    let input = fs::File::open(input_file)?;
    let reader = io::BufReader::new(input);
    let mut output = fs::File::create(output_file)?;

    let mut counters: Vec<usize> = vec![0; 6];
    let mut in_code_block = false;

    for line in reader.lines() {
        let mut line = line?;
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
        }

        if !in_code_block && line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count();
            if level > 0 && level <= 6 {
                counters[level - 1] += 1;
                for counter in counters.iter_mut().skip(level) {
                    *counter = 0;
                }
                let chapter_number: String = counters
                    .iter()
                    .take(level)
                    .filter(|&&c| c > 0)
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(".");
                line = format!(
                    "{} {} {}",
                    "#".repeat(level),
                    chapter_number,
                    line[level..].trim()
                );
            }
        }
        writeln!(output, "{}", line)?;
    }

    Ok(())
}

/// Removes chapter numbers from headlines in a Markdown file.
fn remove_chapter_numbers(input_file: &str, output_file: &str) -> io::Result<()> {
    let input = fs::File::open(input_file)?;
    let reader = io::BufReader::new(input);
    let mut output = fs::File::create(output_file)?;

    let mut in_code_block = false;

    for line in reader.lines() {
        let mut line = line?;
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
        }

        if !in_code_block && line.starts_with('#') {
            // Find the position of the first non-numeric, non-dot character after the `#` prefix
            let level = line.chars().take_while(|&c| c == '#').count();
            if let Some(pos) = line[level..]
                .trim()
                .find(|c: char| !(c.is_numeric() || c == '.'))
            {
                // Remove chapter number (up to the first non-numeric character)
                line = format!(
                    "{} {}",
                    "#".repeat(level),
                    line[level..].trim()[pos..].trim()
                );
            }
        }

        writeln!(output, "{}", line)?;
    }

    Ok(())
}


