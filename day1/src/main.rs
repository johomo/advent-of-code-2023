use std::io;
use std::io::Read;

/// Returns a String with the contents read from standard input.
fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}

/// Replaces a word representing a digit into a digit.
fn replace_from_word_into_number(word: &str) -> &str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => word,
    }
}

fn main() {
    // One may think that the problem can be solved with regex, replacing words with digits (for
    // example, replacing "two" with "2").
    // There are many examples that make the use of regex utterly inconvenient. For instance,
    // replacements with regex are not convenient if haystack contains overlapping patterns.
    // For example, calibration values of "z7twonezp" is "71", but regex may replace "two" with "2",
    // yielding the incorrect calibration value "72".
    // This issue may be solved with somewhat convoluted patterns using lookahead/lookbehind
    // assertions, but Rust's de facto [regex Crate](https://docs.rs/regex/latest/regex/) does not
    // support them.

    let input = read_from_stdin();

    // Use this vector of patterns to answer the first question.
    // let patterns = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    // Use this vector of patterns to answer the second question.
    let patterns = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
        "4", "5", "6", "7", "8", "9",
    ];

    let sum: isize = input
        .split('\n')
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let mut digits_indices = patterns
                .iter()
                .flat_map(|&pattern| line.match_indices(pattern).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            digits_indices.sort_by_key(|match_index| match_index.0);
            let mut digits = digits_indices.iter().map(|match_index| match_index.1);
            let first = replace_from_word_into_number(digits.next().unwrap_or("0"));
            let second = replace_from_word_into_number(digits.last().unwrap_or(first));
            format!("{}{}", first, second).parse::<isize>().unwrap()
        })
        .sum();
    println!("{:?}", sum);
}
