use clap::Parser;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SchematicItem {
    origin: Point,
    num_str: String,
}

impl SchematicItem {
    fn is_part_number(&self) -> bool {
        self.num_str.chars().all(|c| c.is_ascii_digit())
    }

    fn gen_neighbors(&self) -> HashSet<Point> {
        let mut neighbors: HashSet<Point> = HashSet::new();
        let item_points: Vec<Point> = (0..self.num_str.len())
            .map(|n| Point {
                x: self.origin.x + n as i32,
                y: self.origin.y,
            })
            .collect();

        for point in item_points {
            neighbors.insert(Point {
                x: point.x - 1,
                y: point.y - 1,
            });
            neighbors.insert(Point {
                x: point.x - 1,
                y: point.y + 0,
            });
            neighbors.insert(Point {
                x: point.x - 1,
                y: point.y + 1,
            });
            neighbors.insert(Point {
                x: point.x + 0,
                y: point.y - 1,
            });
            neighbors.insert(Point {
                x: point.x + 0,
                y: point.y + 1,
            });
            neighbors.insert(Point {
                x: point.x + 1,
                y: point.y - 1,
            });
            neighbors.insert(Point {
                x: point.x + 1,
                y: point.y + 0,
            });
            neighbors.insert(Point {
                x: point.x + 1,
                y: point.y + 1,
            });
        }
        neighbors
    }
}

/// Advent of Code - Day 3a
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to puzzle input
    input: String,
}
fn main() {
    let cli_args = Args::parse();

    let file_input = File::open(cli_args.input.as_str())
        .expect(format!("Could not open file {}", cli_args.input).as_str());
    let file_reader = BufReader::new(file_input);

    let file_lines = file_reader.lines();

    let mut part_numbers: Vec<SchematicItem> = Vec::new();
    let mut symbols: HashSet<Point> = HashSet::new();
    for (n, line) in file_lines.enumerate() {
        if let Ok(line) = line {
            let items = parse_input(&line, n.try_into().unwrap());

            for symbol in items.iter().filter(|item| !item.is_part_number()) {
                symbols.insert(symbol.origin);
            }

            for part_number in items.iter().filter(|item| item.is_part_number()) {
                part_numbers.push(part_number.to_owned());
            }
        }
    }

    let total_part_numbers: u32 = part_numbers
        .iter()
        .map(|p| {
            if p.gen_neighbors()
                .iter()
                .any(|point| symbols.contains(point))
            {
                p.num_str.parse().unwrap()
            } else {
                0
            }
        })
        .sum();
    println!("Total Part Number: {total_part_numbers}");
}

fn parse_input(line: &str, n_line: i32) -> Vec<SchematicItem> {
    let part_num_re = Regex::new(r"([0-9]+)").unwrap();
    let sym_re = Regex::new(r"([^0-9\.])").unwrap();

    let mut items: Vec<SchematicItem> = Vec::new();
    for m in part_num_re.find_iter(line) {
        items.push(SchematicItem {
            origin: Point {
                x: m.start() as i32,
                y: n_line,
            },
            num_str: m.as_str().to_string(),
        })
    }
    for m in sym_re.find_iter(line) {
        items.push(SchematicItem {
            origin: Point {
                x: m.start() as i32,
                y: n_line,
            },
            num_str: m.as_str().to_string(),
        })
    }

    items
}
