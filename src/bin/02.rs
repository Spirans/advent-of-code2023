use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, digit0, space0},
    combinator::map_res,
    multi::separated_list0,
    sequence::{pair, preceded, separated_pair},
    IResult, ParseTo, Parser,
};

advent_of_code::solution!(2);

fn parse_line(input: &str) -> IResult<&str, Vec<Vec<(u32, &str)>>> {
    preceded(parse_game, parse_grabs)(input)
}

fn parse_game(input: &str) -> IResult<&str, ((&str, &str), &str)> {
    pair(separated_pair(tag("Game"), space0, digit0), tag(":"))(input)
}

fn parse_cube(input: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(map_res(digit0, |v: &str| v.parse()), space0, alpha0)(input)
}

fn parse_grab(input: &str) -> IResult<&str, Vec<(u32, &str)>> {
    separated_list0(tag(","), preceded(space0, parse_cube))(input)
}

fn parse_grabs(input: &str) -> IResult<&str, Vec<Vec<(u32, &str)>>> {
    separated_list0(tag(";"), parse_grab)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let limits: HashMap<&str, u32> = vec![("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    let mut result = 0;
    for (number, line) in input.lines().enumerate() {
        let (_, grabs) = parse_line(line).ok()?;
        if !grabs
            .into_iter()
            .flatten()
            .any(|(count, color)| limits[color] < count)
        {
            result += number as u32 + 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let limits: HashMap<&str, u32> = vec![("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    let mut result = 0;
    for line in input.lines() {
        let (_, grabs) = parse_line(line).ok()?;
        let mut maximums: HashMap<&str, u32> = vec![("red", 0), ("green", 0), ("blue", 0)]
            .into_iter()
            .collect();
        grabs.into_iter().flatten().for_each(|(count, color)| {
            if maximums[color] < count {
                maximums.insert(color, count);
            }
        });
        result += maximums.values().fold(1, |acc, &x| acc * x);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part_one(input);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part_two(input);
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game(input);
        assert_eq!(
            result,
            Ok((
                " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                (("Game", "1"), ":")
            ))
        );
    }

    #[test]
    fn test_parse_cube() {
        let input = "3 blue";
        let result = parse_cube(input);
        assert_eq!(result, Ok(("", (3, "blue"))))
    }

    #[test]
    fn test_parse_grab() {
        let input = "1 red, 2 green, 6 blue";
        let result = parse_grab(input);
        assert_eq!(
            result,
            Ok(("", vec![(1, "red"), (2, "green"), (6, "blue")]))
        )
    }

    #[test]
    fn test_parse_grabs() {
        let input = " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_grabs(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    vec![(3, "blue"), (4, "red")],
                    vec![(1, "red"), (2, "green"), (6, "blue")],
                    vec![(2, "green")]
                ]
            ))
        )
    }

    #[test]
    fn test_parse_line() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_line(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    vec![(3, "blue"), (4, "red")],
                    vec![(1, "red"), (2, "green"), (6, "blue")],
                    vec![(2, "green")]
                ]
            ))
        )
    }
}
