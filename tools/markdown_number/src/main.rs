use std::fs;
use std::env;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {

    // Input and output file paths
    let input_file = env::args().skip(1).next().unwrap();
    let output_file = "output.md";

    // Open the input file
    let input = fs::File::open(input_file)?;
    let reader = io::BufReader::new(input);

    // Prepare the output file
    let mut output = fs::File::create(output_file)?;

    // Chapter counters for different headline levels
    let mut counters: Vec<usize> = vec![0; 6];

    // Track whether we're inside a code block
    let mut in_code_block = false;

    for line in reader.lines() {
        let mut line = line?;

        // Detect the start or end of a code block
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
        }

        // Process headlines only when not inside a code block
        if !in_code_block && line.starts_with('#') {
            // Count the number of '#' characters to determine the headline level
            let level = line.chars().take_while(|&c| c == '#').count();

            if level > 0 && level <= 6 {
                // Increment the counter for the current level
                counters[level - 1] += 1;

                // Reset counters for deeper levels
                for counter in counters.iter_mut().skip(level) {
                    *counter = 0;
                }

                // Generate the chapter number
                let chapter_number: String = counters
                    .iter()
                    .take(level)
                    .filter(|&&c| c > 0)
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(".");

                // Rewrite the headline with the chapter number
                line = format!(
                    "{} {} {}",
                    "#".repeat(level),
                    chapter_number,
                    line[level..].trim()
                );
            }
        }

        // Write the modified or unmodified line to the output file
        writeln!(output, "{}", line)?;
    }

    println!("Processed file written to {}", output_file);
    Ok(())
}

