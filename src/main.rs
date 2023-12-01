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
    let mut line_num = 1;
    while let Some(Ok(mut line)) = file_lines.next() {
        let input = line.clone();
        line = line.to_ascii_lowercase();

        let words_to_digits = vec![
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ];

        // Find first and last digits
        let mut digits: Vec<char> = Vec::new();
        while !line.is_empty() {
            if let Some(c) = line.chars().next() {
                let digit: Option<char> = if c.is_ascii_digit() {
                    // Found ASCII digit -> remove from input and return
                    line = line.strip_prefix(c).unwrap().to_string();
                    Some(c)
                } else {
                    // Look for digits as words
                    let mut words = (&words_to_digits).into_iter();
                    loop {
                        if let Some((word, digit)) = words.next() {
                            if let Some(new_line) = line.strip_prefix(word) {
                                // Found digit as word -> remove from input and return with
                                // last char of word prepended in case it is needed for next
                                // word
                                line = if let Some(c) = word.chars().last() {
                                    format!("{c}{new_line}")
                                } else {
                                    new_line.to_string()
                                };
                                break Some(*digit);
                            }
                        } else {
                            // No digits as words found -> remove first char
                            line.drain(..1);
                            break None;
                        }
                    }
                };

                if let Some(d) = digit {
                    digits.push(d);
                }
            }
        }

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
