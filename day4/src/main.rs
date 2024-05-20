use itertools::Itertools;
use std::collections::HashSet;
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

    let matches: Vec<usize> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(':').last().unwrap())
        .map(|line| {
            let (left, right) = line.split('|').collect_tuple().unwrap();
            let numbers_you_have: HashSet<usize> = left
                .split(' ')
                .filter(|token| !token.is_empty())
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            let winning_numbers: HashSet<usize> = right
                .split(' ')
                .filter(|token| !token.is_empty())
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            numbers_you_have
                .intersection(&winning_numbers)
                .collect::<Vec<_>>()
                .len()
        })
        .collect();

    let sum_of_points: usize = matches
        .iter()
        .map(|&matches| {
            // Method usize::pow expects exponent to be of type u32.
            let matches: u32 = matches.try_into().unwrap();
            match matches {
                0 => 0,
                n => usize::pow(2, n - 1),
            }
        })
        .sum();
    println!(
        "Answer to first question: The sum of points is {}",
        sum_of_points
    );

    let mut cards = vec![1; matches.len()];
    for card in 0..cards.len() {
        for increment in 0..matches[card] {
            cards[card + 1 + increment] += cards[card];
        }
    }
    let total_number_of_cards: usize = cards.iter().sum();
    println!(
        "Answer to second question: The total number of cards is {}",
        total_number_of_cards
    );
}
