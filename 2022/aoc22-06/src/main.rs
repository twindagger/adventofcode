use std::collections::VecDeque;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_raw(part1, part2)
}

fn index_of(buffer: &VecDeque<char>, c: &char) -> Option<usize> {
    for (ix, item) in buffer.iter().enumerate() {
        if item == c {
            return Some(ix);
        }
    }

    None
}

fn find_pos(contents: &str, marker_len: usize) -> Result<usize> {
    let mut buffer: VecDeque<char> = VecDeque::new();

    for (pos, c) in contents.chars().enumerate() {
        match index_of(&buffer, &c) {
            Some(ix) => {
                // this rotates characters moving the last occurrence of the repeated character to
                // the end of the ring buffer, then removes them
                //
                // ex: buffer=abcdef with current character d (ix=3)
                buffer.rotate_left(ix + 1);
                // buffer=efabcd
                buffer.resize(buffer.len() - ix - 1, ' ');
                // buffer=ef
            }
            None => (),
        }
        buffer.push_back(c);
        if buffer.len() == marker_len {
            return Ok(pos + 1);
        }
    }

    bail!("no position found");
}

fn part1(contents: &str) -> Result<usize> {
    find_pos(contents, 4)
}

fn part2(contents: &str) -> Result<usize> {
    find_pos(contents, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?);
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz")?);
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg")?);
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?);
        assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?);
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz")?);
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg")?);
        assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?);
        assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?);

        Ok(())
    }
}
