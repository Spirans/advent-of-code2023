advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for line in input.lines() {
        let first_digit_index = line.find(char::is_numeric)?;
        let last_digit_index = line.rfind(char::is_numeric)?;
        let first_digit = line.chars().nth(first_digit_index)?.to_digit(10)?;
        let last_digit = line.chars().nth(last_digit_index)?.to_digit(10)?;
        result += first_digit * 10 + last_digit;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    let mapping: Vec<(u32, &str)> = (1u32..)
        .zip(vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ])
        .collect();

    for line in input.lines() {
        let first_word_digit = mapping
            .iter()
            .filter_map(|&(val, word)| line.find(word).map(|idx| (idx, val)))
            .min();
        let last_word_digit = mapping
            .iter()
            .filter_map(|&(val, word)| line.rfind(word).map(|idx| (idx, val)))
            .max();

        let digits: Vec<(usize, char)> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_digit())
            .collect();

        let first_digit = digits
            .first()
            .map(|&(idx, val)| (idx, val.to_digit(10).unwrap_or(0)));
        let last_digit = digits
            .last()
            .map(|&(idx, val)| (idx, val.to_digit(10).unwrap_or(0)));

        let first = match (first_word_digit, first_digit) {
            (None, None) => 0,
            (Some((_, v)), None) | (None, Some((_, v))) => v,
            (Some((l_idx, l_v)), Some((r_idx, r_v))) => {
                if l_idx < r_idx {
                    l_v
                } else {
                    r_v
                }
            }
        };
        let last = match (last_word_digit, last_digit) {
            (None, None) => 0,
            (Some((_, v)), None) | (None, Some((_, v))) => v,
            (Some((l_idx, l_v)), Some((r_idx, r_v))) => {
                if l_idx > r_idx {
                    l_v
                } else {
                    r_v
                }
            }
        };

        result += first * 10 + last;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let result = part_one(input);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = part_two(input);
        assert_eq!(result, Some(281));
    }
}
