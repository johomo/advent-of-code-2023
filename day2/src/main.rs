use itertools::Itertools;
use regex::{Captures, Regex};
use std::io;
use std::io::Read;

/// Returns a String with the contents read from standard input.
fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let input = read_from_stdin();
    let pattern = Regex::new("(?<number>[0-9]+) (?<colour>(blue|green|red))").unwrap();

    let max_cubes_per_game: Vec<Vec<isize>> = input
        .split('\n')
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let mut captures_sorted_by_colour =
                pattern.captures_iter(line).collect::<Vec<Captures>>();
            captures_sorted_by_colour.sort_by_key(|capture| String::from(&capture["colour"]));
            let max_rolls = captures_sorted_by_colour
                .into_iter()
                .group_by(|capture| String::from(&capture["colour"]))
                .into_iter()
                .map(|(colour, group)| (colour, group.collect::<Vec<_>>()))
                .collect::<Vec<_>>()
                .into_iter()
                .map(|(_, captures_vec)| {
                    captures_vec
                        .into_iter()
                        .map(|capture| capture["number"].parse::<isize>().unwrap())
                        .max()
                        .unwrap()
                })
                .collect::<Vec<isize>>();
            max_rolls
        })
        .collect::<Vec<Vec<isize>>>();

    // Answer to the first question
    let cubes_available = [14, 13, 12]; // (blue, green, red)
    let sum_of_ids_of_valid_games: usize = max_cubes_per_game
        .iter()
        .enumerate()
        .filter(|(_, max_cubes)| {
            max_cubes[0] <= cubes_available[0]
                && max_cubes[1] <= cubes_available[1]
                && max_cubes[2] <= cubes_available[2]
        })
        .map(|(game_id, _)| game_id + 1)
        .sum();
    println!(
        "Answer to second question: The sum of IDs of valid games is {}",
        sum_of_ids_of_valid_games
    );

    // Answer to the second question
    let power: isize = max_cubes_per_game
        .iter()
        .map(|max_cubes| max_cubes[0] * max_cubes[1] * max_cubes[2])
        .sum();
    println!("Answer to second question: Power is {}", power);
}
