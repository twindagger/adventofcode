use aoc_common::*;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

fn main() {
    run_vec(parse, part1, part2);
}

fn parse(contents: &str) -> Vec<String> {
    contents.lines().map(|x| x.to_string()).collect()
}

fn escape(line: &str) -> String {
    format!("\"{}\"", line.replace("\\", "\\\\").replace("\"", "\\\\"))
}

fn unescape(line: &str) -> String {
    lazy_static! {
        static ref ASCII_ESCAPE: Regex = Regex::new("\\\\x([0-9a-f]{2})").unwrap();
    }

    let line = &line[1..line.len() - 1]; // strip outer quotes
    let line = line.replace("\\\"", "\"").replace("\\\\", "\\"); //unescape \" and \\

    ASCII_ESCAPE
        .replace_all(&line, |caps: &Captures| {
            char::from_u32(u32::from_str_radix(&caps[1], 16).unwrap())
                .unwrap()
                .to_string()
        })
        .to_string()
}

fn part1(contents: &[String]) -> usize {
    contents
        .iter()
        .map(|line| line.chars().count() - unescape(line).chars().count())
        .sum()
}

fn part2(contents: &[String]) -> usize {
    contents
        .iter()
        .map(|line| escape(line).chars().count() - line.chars().count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let result = part1(&parse(SAMPLE));

        assert_eq!(result, 12);
    }

    #[test]
    fn sample_part2() {
        let result = part2(&parse(SAMPLE));

        assert_eq!(result, 19);
    }

    const SAMPLE: &str = include_str!("sample.in");
}