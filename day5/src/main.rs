use std::cmp;
use std::io;
use std::io::Read;

/// Returns a String with the contents read from standard input.
fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}

/// Parse digits as numbers from a string.
///
/// The string may contain other characters different from digits.
/// ```
/// let actual = parse_numbers("There are 45 cats on top of 3 walls and only 1 of them is white.");
/// let expected = vec![45, 3, 1];
/// assert_eq!(expected, actual);
/// ```
fn parse_numbers(line: &str) -> Vec<u64> {
    line.split(' ')
        .filter(|&item| !item.is_empty())
        .filter_map(|seed_number| seed_number.parse().ok())
        .collect()
}

#[derive(Clone)]
struct MapRule {
    source_range: (u64, u64),
    destination: u64,
}

struct Map {
    rules: Vec<MapRule>,
}

impl Map {
    fn from_iter(iter: impl IntoIterator<Item = MapRule>) -> Self {
        let mut rules_vec = Vec::<MapRule>::new();
        for rule in iter {
            rules_vec.push(rule.clone());
        }
        Map { rules: rules_vec }
    }

    fn convert(&self, number: u64) -> u64 {
        for item in self.rules.iter() {
            if item.source_range.0 <= number && number <= item.source_range.1 {
                return item.destination + (number - item.source_range.0);
            }
        }
        return number;
    }
}

fn apply_maps(number: u64, maps: &Vec<Map>) -> u64 {
    let mut new = number;
    for map in maps {
        new = map.convert(new);
    }
    new
}

fn main() {
    let input = read_from_stdin();
    let input: Vec<&str> = input.split("\n\n").collect();

    // Parse maps
    let maps: Vec<Map> = input
        .iter()
        .skip(1)
        .map(|&ruleset| {
            Map::from_iter(
                ruleset
                    .split("\n")
                    .filter(|&rule| !rule.is_empty())
                    .skip(1)
                    .map(|rule| {
                        let rule = parse_numbers(rule);
                        MapRule {
                            destination: rule[0],
                            source_range: (rule[1], rule[1] + rule[2]),
                        }
                    }),
            )
        })
        .collect();

    // To answer the first question, set chunks to 1.
    // To answer the second question, set chunks to 2.
    // Execution panics with any other value of chunks.
    // Second answer takes around 25 min in my laptop.
    // Clever strategies can be implemented so that less
    // numbers are scanned.
    let chunks: usize = 2;
    let closest_seed: u64 = parse_numbers(input[0])
        .chunks(chunks)
        .flat_map(|pairs| {
            let from = pairs[0];
            let to = match chunks {
                1 => pairs[0] + 1,
                2 => pairs[0] + pairs[1],
                _ => panic!("Unexpected value of chunks {}", chunks),
            };
            from..to
        })
        .reduce(|closest_location, number| {
            cmp::min(closest_location, apply_maps(number, &maps))
        })
        .unwrap();
    println!("{:?}", closest_seed);
}
