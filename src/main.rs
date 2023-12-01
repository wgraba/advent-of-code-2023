use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    input: String,
}
fn main() {
    let cli_args = Args::parse();

    let file_input = File::open(cli_args.input.as_str())
        .expect(format!("Could not open file {}", cli_args.input).as_str());
    let file_reader = BufReader::new(file_input);

    let mut file_lines = file_reader.lines();

    let mut total: u32 = 0;
    while let Some(Ok(line)) = file_lines.next() {
        let first_digit = line.chars().find(|c| c.is_ascii_digit());
        let last_digit = line.chars().rfind(|c| c.is_ascii_digit());

        match (first_digit, last_digit) {
            (Some(first), Some(last)) => {
                let num_str = format!("{first}{last}");
                let num: u8 = num_str
                    .parse()
                    .expect(format!("Unable to convert {num_str}").as_str());

                println!("Found number: {num}");
                total += num as u32;
            }

            _ => eprint!("Could not find first or last ASCII digit in '{line}'"),
        }
    }

    println!("File total: {total}");
}
