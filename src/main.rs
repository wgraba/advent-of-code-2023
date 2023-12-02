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

    let words_to_digits = vec![
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "t3e"),
            ("four", "f4r"),
            ("five", "f5e"),
            ("six", "s6x"),
            ("seven", "s7n"),
            ("eight", "e8t"),
            ("nine", "n9e"),
        ];

    let file_input = File::open(cli_args.input.as_str())
        .expect(format!("Could not open file {}", cli_args.input).as_str());
    let file_reader = BufReader::new(file_input);

    let mut file_lines = file_reader.lines();

    let mut total: u32 = 0;
    let mut line_num = 1;
    while let Some(Ok(mut line)) = file_lines.next() {
        let input = line.clone();
        line = line.to_ascii_lowercase();

        // Find first and last digits
        for (word, digit) in &words_to_digits {
            line = line.replace(word, digit);
        }

        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let first_digit = digits.first();
        let last_digit = digits.last();
        match (first_digit, last_digit) {
            (Some(first), Some(last)) => {
                let num_str = format!("{first}{last}");
                let num: u8 = num_str
                    .parse()
                    .expect(format!("Unable to convert {num_str}").as_str());

                total += num as u32;

                println!("{line_num}: {input} -> {digits:?} -> {num}");
                line_num += 1;
            }

            _ => panic!("Could not find first or last ASCII digit in '{line}'"),
        }
    }

    println!("File total: {total}");
}
