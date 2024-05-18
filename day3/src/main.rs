use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::io::Read;

/// Return a String with the contents read from standard input.
fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}

/// Rectangle spans an area in a grid.
#[derive(PartialEq, Eq, Hash)]
struct Rectangle {
    /// Location of the upper left corner in a grid.
    /// (horizontal axis, vertical axis)
    start: (isize, isize),
    /// Location of the bottom right corner in a grid.
    /// /// (horizontal axis, vertical axis)
    end: (isize, isize),
}

impl Rectangle {
    /// Get the coordinates of the hull of this rectangle. The hull of a rectangle R is defined
    /// as the smallest rectangle different from R that completely contains R.
    pub fn get_hull_rectangle(&self) -> Rectangle {
        Rectangle {
            start: (self.start.0 - 1, self.start.1 - 1),
            end: (self.end.0 + 1, self.end.1 + 1),
        }
    }

    /// Returns true if this rectangle overlaps with another rectangle.
    pub fn overlaps_with(&self, r: &Rectangle) -> bool {
        let r_is_up = r.end.1 < self.start.1;
        let r_is_down = self.end.1 < r.start.1;
        let r_is_to_the_left = r.end.0 < self.start.0;
        let r_is_to_the_right = self.end.0 < r.start.0;
        !r_is_up && !r_is_down && !r_is_to_the_left && !r_is_to_the_right
    }
}

/// Token represents either a number or a symbol and its location in the grid.
#[derive(PartialEq, Eq, Hash)]
struct Token {
    /// The value of the token. Either a number or a symbol.
    value: String,
    /// The location of the token in the grid.
    coordinates: Rectangle,
}

/// Get tokens adjacent to a token.
fn get_adjacent_tokens<'b>(t: &Token, tokens: &'b HashSet<Token>) -> HashSet<&'b Token> {
    let t_hull = t.coordinates.get_hull_rectangle();
    tokens
        .iter()
        .filter(|&token| t_hull.overlaps_with(&token.coordinates))
        .collect()
}

fn main() {
    let input = read_from_stdin();
    let grid: Vec<&str> = input.split('\n').collect();

    // Parse numbers
    let re = Regex::new("[0-9]+").unwrap();
    let numbers: HashSet<Token> = grid
        .iter()
        .enumerate()
        .flat_map(|(row_no, &row)| {
            let row_no = row_no as isize;
            re.find_iter(row).map(move |m| Token {
                value: m.as_str().to_string(),
                coordinates: Rectangle {
                    start: (m.start() as isize, row_no),
                    // The end of the match corresponds to the byte immediately following
                    // the last byte in the match. Thus, we substract 1.
                    end: (m.end() as isize - 1, row_no),
                },
            })
        })
        .collect();

    // Parse tokens
    let symbols: HashSet<Token> = grid
        .iter()
        .enumerate()
        .flat_map(|(row_no, &row)| {
            let row_no = row_no as isize;
            row.chars().enumerate().filter_map(move |(char_no, ch)| {
                let char_no = char_no as isize;
                if ch.is_ascii_digit() || ch == '.' {
                    // Numbers and character '.' are not considered symbols.
                    return None;
                }
                Some(Token {
                    value: ch.to_string(),
                    coordinates: Rectangle {
                        start: (char_no, row_no),
                        end: (char_no, row_no),
                    },
                })
            })
        })
        .collect();

    // Answer first question
    let sum: isize = numbers
        .iter()
        .filter(|&number| !get_adjacent_tokens(number, &symbols).is_empty())
        .map(|number| number.value.parse::<isize>().unwrap())
        .sum();
    println!(
        "Answer to first question: The sum of numbers adjacent to any symbol is {}",
        sum
    );

    // Answer second question
    let sum_of_gear_ratios: isize = symbols
        .iter()
        .filter(|&symbol| symbol.value == "*")
        .filter_map(|symbol| {
            let adjacent_numbers = get_adjacent_tokens(symbol, &numbers);
            if adjacent_numbers.len() != 2 {
                return None;
            }
            let gear_ratio = adjacent_numbers
                .iter()
                .map(|number| number.value.parse::<isize>().unwrap())
                .reduce(|first, second| first * second)
                .unwrap();
            Some(gear_ratio)
        })
        .sum();
    println!(
        "Answer to second question: The sum of gear ratios is {}",
        sum_of_gear_ratios
    );
}
