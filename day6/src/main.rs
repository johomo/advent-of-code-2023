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

/// Get the number of holding times that beat the distance record.
fn get_number_of_holding_times_that_beat_distance_record(
    race_time: u64,
    distance_record: u64,
) -> u64 {
    // Let t be the holding time. It verifies 0 <= T <= race_time.
    // The velocity of the boat is a function v(t) = t
    // The travels distance of the boat is distance(t) = v(t) * (race_time - t)
    // We are looking for the values of t such that
    // distance(t) = v(t) * (race_time - t) > distance_record.
    // Notice the strict inequality. The problem does not accept record ties.
    // Equivalently: t * (race_time - t) > distance_record.
    // Equivalently: -t^2 + race_time * t - distance_record > 0
    // Let t_1, t_2 be the two solutions of the equation. Then, the function returns all integers
    // within the open interval ( max(0, t_1), min(t_2, race_time) ).
    // Notice that 0 and race_time are not feasible solutions.

    let race_time = race_time as i64;
    let distance_record = distance_record as i64;
    let discriminant = race_time * race_time - 4 * (-1) * (-distance_record);

    if discriminant < 0 {
        // There are no holding times that beat the distance record.
        return 0;
    } else if discriminant == 0 {
        // There is only one holding time that beat distance record.
        if race_time % 2 == 0 {
            // The only solution is integer: (race_time / 2)
            return 1;
        } else {
            // The only solution is not integer. It is not a feasible solution.
            return 0;
        }
    }

    let discriminant = discriminant as f64;
    let race_time = race_time as f64;

    let mut t_1 = (-race_time + f64::sqrt(discriminant)) / (-2.0);
    let mut t_2 = (-race_time - f64::sqrt(discriminant)) / (-2.0);

    if t_1.fract() == 0.0 {
        // If t_1 is integer, by holding the button that amount of seconds we tie the record.
        // Record ties must be excluded.
        t_1 += 1.0;
    }
    if t_2.fract() == 0.0 {
        // If t_2 is integer, by holding the button that amount of seconds we tie the record.
        // Record ties must be excluded.
        t_2 -= 1.0;
    }

    let min_value = f64::ceil(f64::max(0.0, t_1)) as u64;
    let max_value = f64::floor(f64::min(t_2, race_time)) as u64;
    if min_value > max_value {
        // There are no holding times that beat the distance record.
        return 0;
    }

    return max_value - min_value + 1;
}

fn main() {
    let input = read_from_stdin();
    let input: Vec<&str> = input.split("\n").filter(|line| !line.is_empty()).collect();

    let race_times = parse_numbers(input[0]);
    let distance_records = parse_numbers(input[1]);

    let solution = std::iter::zip(race_times, distance_records)
        .map(|(time, distance)| {
            get_number_of_holding_times_that_beat_distance_record(time, distance)
        })
        .reduce(|accumulator, element| accumulator * element);

    if let Some(solution) = solution {
        println!(
            "The multiplication of all holding times that beat the distance record is {}",
            solution
        );
    } else {
        println!("There is no way to beat any race record at all.")
    }
}
