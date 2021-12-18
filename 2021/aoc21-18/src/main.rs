#![feature(destructuring_assignment)]

use aoc_common::*;
use std::fmt;
use std::ops::Add;
use std::str::FromStr;

fn main() {
    run(parse, part1, part2);
}

#[derive(Clone, PartialEq, Eq)]
struct FishNumVec {
    contents: Vec<Token>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Separator,
    Number(u32),
}

impl FishNumVec {
    fn plus(&self, other: &FishNumVec) -> FishNumVec {
        let mut contents =
            std::vec::Vec::with_capacity(self.contents.len() + other.contents.len() + 3);
        contents.push(Token::Open);
        contents.append(&mut self.contents.clone());
        contents.push(Token::Separator);
        contents.append(&mut other.contents.clone());
        contents.push(Token::Close);

        let mut result = FishNumVec { contents };

        result.reduce();

        result
    }

    fn reduce(&mut self) {
        while self.reduce_one() {}
    }

    fn reduce_one(&mut self) -> bool {
        let mut depth = 0;
        for i in 0..self.contents.len() {
            match self.contents[i] {
                Token::Open => {
                    depth += 1;
                    if depth == 5 {
                        self.explode(i);
                        return true;
                    }
                }
                Token::Close => depth -= 1,
                _ => (),
            }
        }
        for i in 0..self.contents.len() {
            match self.contents[i] {
                Token::Number(x) if x > 9 => {
                    self.split(i, x);
                    return true;
                }
                _ => (),
            }
        }

        false
    }

    fn explode(&mut self, pos: usize) {
        // pos is the position of the opening bracket, find the closing bracket
        let left;
        if let Token::Number(l) = self.contents[pos + 1] {
            left = l;
        } else {
            panic!("cannot reduce too deeply nested item, left not found");
        }
        let right;
        if let Token::Number(r) = self.contents[pos + 3] {
            right = r;
        } else {
            panic!("cannot reduce too deeply nested item, right not found");
        }

        let mut cur = pos - 1;
        // add left to the next number to the left
        while cur > 0 {
            if let Token::Number(num) = self.contents[cur] {
                self.contents[cur] = Token::Number(num + left);
                break;
            }
            cur -= 1;
        }

        cur = pos + 5;

        // add right to the next number to the right;
        while cur < self.contents.len() {
            if let Token::Number(num) = self.contents[cur] {
                self.contents[cur] = Token::Number(num + right);
                break;
            }
            cur += 1;
        }

        // replace current pair with 0
        self.contents.splice(pos..(pos + 5), vec![Token::Number(0)]);
    }

    fn split(&mut self, pos: usize, num: u32) {
        let left = num / 2;
        let right = num - left;

        self.contents.splice(
            pos..=pos,
            vec![
                Token::Open,
                Token::Number(left),
                Token::Separator,
                Token::Number(right),
                Token::Close,
            ],
        );
    }

    fn magnitude(&self) -> u32 {
        let mut mult = 1;
        let mut result = 0;
        for c in self.contents.iter() {
            match c {
                Token::Open => mult *= 3,
                Token::Close => mult /= 2,
                Token::Separator => mult = mult / 3 * 2,
                Token::Number(num) => result += mult * num,
            }
        }
        result
    }
}

impl Add for FishNumVec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.plus(&other)
    }
}

impl Add<&FishNumVec> for FishNumVec {
    type Output = Self;

    fn add(self, other: &FishNumVec) -> Self {
        self.plus(other)
    }
}

impl std::iter::Sum<FishNumVec> for FishNumVec {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = FishNumVec>,
    {
        let mut acc = iter.next().unwrap();
        for x in iter {
            acc = acc + x;
        }
        acc
    }
}

impl<'a> std::iter::Sum<&'a FishNumVec> for FishNumVec {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = &'a FishNumVec>,
    {
        let mut acc = iter.next().unwrap().clone();
        for x in iter {
            acc = acc + x;
        }
        acc
    }
}
impl FromStr for FishNumVec {
    type Err = ();

    fn from_str(num: &str) -> Result<Self, Self::Err> {
        Ok(FishNumVec {
            contents: num.trim().chars().filter_map(Token::from_char).collect(),
        })
    }
}

impl fmt::Display for FishNumVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.contents
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl fmt::Debug for FishNumVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.contents
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Token {
    fn from_char(c: char) -> Option<Token> {
        match c {
            '[' => Some(Token::Open),
            ']' => Some(Token::Close),
            ',' => Some(Token::Separator),
            '0'..='9' => Some(Token::Number(c.to_string().parse::<u32>().unwrap())),
            ' ' => None,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Open => write!(f, "["),
            Token::Close => write!(f, "]"),
            Token::Separator => write!(f, ","),
            Token::Number(x) => write!(f, "{}", x),
        }
    }
}

fn parse(contents: &str) -> Vec<FishNumVec> {
    contents.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(contents: &Vec<FishNumVec>) -> u32 {
    let sum: FishNumVec = contents.iter().sum();
    sum.magnitude()
}

fn part2(contents: &Vec<FishNumVec>) -> u32 {
    let mut max = 0;
    for x in contents.iter() {
        for y in contents.iter() {
            if x != y {
                let mag = x.plus(y).magnitude();
                if mag > max {
                    max = mag;
                }
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        let lhs: FishNumVec = "[1,2]".parse().unwrap();
        let rhs: FishNumVec = "[[3,4],5]".parse().unwrap();

        assert_eq!(lhs + rhs, "[[1,2],[[3,4],5]]".parse().unwrap());
    }

    #[test]
    fn explode_reduce() {
        let mut num: FishNumVec = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        num.reduce();
        assert_eq!(num, "[[[[0,9],2],3],4]".parse().unwrap());

        let mut num: FishNumVec = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        num.reduce();
        assert_eq!(num, "[7,[6,[5,[7,0]]]]".parse().unwrap());

        let mut num: FishNumVec = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        num.reduce();
        assert_eq!(num, "[[6,[5,[7,0]]],3]".parse().unwrap());

        let mut num: FishNumVec = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        num.reduce_one();
        assert_eq!(num, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap());

        let mut num: FishNumVec = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        num.reduce_one();
        assert_eq!(num, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap());
    }

    #[test]
    fn complicated_reduce() {
        let lhs: FishNumVec = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let rhs: FishNumVec = "[1,1]".parse().unwrap();

        assert_eq!(
            lhs + rhs,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap()
        );
    }

    #[test]
    fn simple_sum() {
        let nums = parse(SIMPLE_SUM);

        assert_eq!(
            nums.into_iter().sum::<FishNumVec>(),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap()
        );
    }

    #[test]
    fn harder_sum() {
        let nums = parse(HARDER_SUM);

        assert_eq!(
            nums.into_iter().sum::<FishNumVec>(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn magnitude() {
        let num: FishNumVec = "[9,1]".parse().unwrap();
        assert_eq!(num.magnitude(), 29);

        let num: FishNumVec = "[[1,2],[[3,4],5]]".parse().unwrap();
        assert_eq!(num.magnitude(), 143);

        let num: FishNumVec = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(num.magnitude(), 1384);

        let num: FishNumVec = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
        assert_eq!(num.magnitude(), 445);

        let num: FishNumVec = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();
        assert_eq!(num.magnitude(), 791);

        let num: FishNumVec = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
        assert_eq!(num.magnitude(), 1137);

        let num: FishNumVec = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();
        assert_eq!(num.magnitude(), 3488);
    }

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 4140);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 3993);
    }

    const SIMPLE_SUM: &str = "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
";
    const HARDER_SUM: &str = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";

    const SAMPLE: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
}
