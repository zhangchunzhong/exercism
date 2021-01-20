use itertools::Itertools;
use std::char;
use std::collections::{HashMap, HashSet};

fn check_equation(puzzle: &str, solution: &HashMap<char, u8>) -> bool {
    let pstr: String = puzzle
        .chars()
        .map(|c| {
            if let Some(&n) = solution.get(&c) {
                char::from_digit(n as u32, 10).unwrap()
            } else {
                c
            }
        })
        .collect();
    let just_numbers = pstr
        .chars()
        .filter(|c| c.is_digit(10) || c.is_whitespace())
        .collect::<String>();
    let no_leading_zeroes = just_numbers
        .split_whitespace()
        .all(|number| number.chars().next().unwrap() != '0');
    if no_leading_zeroes == false {
        return false;
    }
    let equation: Vec<&str> = pstr.split("==").collect();
    let left: u64 = equation[0]
        .split('+')
        .map(str::trim)
        .map(|x| x.parse::<u64>().unwrap())
        .sum();
    let right: u64 = equation[1].trim().parse::<u64>().unwrap();

    left == right
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters: HashSet<char> = input
        .chars()
        .filter(|&c| c.is_alphabetic() && c.is_uppercase())
        .collect();
    let letters: Vec<char> = letters.into_iter().collect();
    let len = letters.len();
    let combins = (0..=9).permutations(len);
    for c in combins {
        let mut chk = HashMap::new();
        let mut i = 0;
        for l in letters.clone() {
            chk.insert(l, c[i]);
            i += 1;
        }
        if check_equation(input, &chk) {
            return Some(chk);
        }
    }
    None
}
