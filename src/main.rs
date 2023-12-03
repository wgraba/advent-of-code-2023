use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Round {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to puzzle input
    input: String,
}
fn main() {
    let cli_args = Args::parse();

    let max_round = Round {
        num_blue: 14,
        num_green: 13,
        num_red: 12,
    };

    let file_input = File::open(cli_args.input.as_str())
        .expect(format!("Could not open file {}", cli_args.input).as_str());
    let file_reader = BufReader::new(file_input);

    let mut file_lines = file_reader.lines();

    let mut possible_game_total: u32 = 0;
    while let Some(Ok(line)) = file_lines.next() {
        let game = parse_game_input(&line);
        let mut is_possible = false;
        for round in game.rounds {
            is_possible = round.num_blue <= max_round.num_blue
                && round.num_green <= max_round.num_green
                && round.num_red <= max_round.num_red;
            if !is_possible {
                break;
            }
        }

        if is_possible {
            possible_game_total += game.id;
        }

        println!(
            "Game {0:04}: {1}",
            game.id,
            if is_possible {
                "Possible"
            } else {
                "Impossible"
            }
        );
    }

    println!("Total Possible Game IDs: {possible_game_total}");
}

fn parse_game_input(input: &str) -> Game {
    let game_id: u32 = input
        .split_once(":")
        .unwrap()
        .0
        .split_once(" ")
        .unwrap()
        .1
        .parse()
        .expect("Game ID is not a number");
    let rounds_str = input.split_once(":").unwrap().1.split(";");

    let mut num_red: u32 = 0;
    let mut num_blue: u32 = 0;
    let mut num_green: u32 = 0;
    let mut rounds: Vec<Round> = Vec::new();
    for round in rounds_str {
        let balls = round.split(",");
        for ball in balls {
            let (num_balls_str, ball_color) = ball.trim().split_once(" ").unwrap();
            let num_balls: u32 = num_balls_str
                .parse()
                .expect(&format!("Game {game_id}: unable to parse number of balls."));
            match ball_color {
                "red" => num_red = num_balls,
                "blue" => num_blue = num_balls,
                "green" => num_green = num_balls,
                _ => panic!("Unknown ball color '{ball_color}'"),
            };
        }
        rounds.push(Round {
            num_red: num_red,
            num_blue: num_blue,
            num_green: num_green,
        });
    }
    Game {
        id: game_id,
        rounds: rounds,
    }
}
