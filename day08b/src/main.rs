use std::collections::{HashSet};
use itertools::Itertools;

fn decode_signal(word: &str, mapping: &Vec<HashSet<char>>) -> Option<usize> {
    let word_set: HashSet<char> = word.chars().collect();
    if word.len() == 2 {
        return Some(1)
    } else if word.len() == 3 {
        return Some(7)
    } else if word.len() == 4 {
        return Some(4)
    } else if word.len() == 5 {
        return if word_set.is_superset(&mapping[7]) {
            Some(3)
        } else if word_set.intersection(&mapping[4]).collect_vec().len() == 3 {
            Some(5)
        } else {
            Some(2)
        }
    } else if word.len() == 6 {
        return if word_set.is_superset(&mapping[4]) {
            Some(9)
        } else if word_set.is_superset(&mapping[1]) {
            Some(0)
        } else {
            Some(6)
        }
    } else if word.len() == 7 {
        return Some(8)
    } else {
        None
    }
}

fn main() {
    let input: Vec<(&str, &str)> = include_str!("../input.txt")
        .lines().map(|x| (x.split(" | ").nth(0).unwrap(), x.split(" | ").nth(1).unwrap())).collect();

    let mut total = 0;
    for signal_pair in &input {
        let empty_map = HashSet::new();
        let mut mapping: Vec<HashSet<char>> = vec![empty_map; 10];
        let mut encryption: Vec<&str> = signal_pair.0.split(" ").collect();
        let signal: Vec<&str> = signal_pair.1.split(" ").collect();
        encryption.sort_by(|a, b| a.len().cmp(&b.len()));
        for i in 0..encryption.len() {
            let word = encryption[i];
            let word_set: HashSet<char> = word.chars().collect();
            let decode = decode_signal(word, &mapping);
            mapping[decode.unwrap()] = word_set.clone()
        }

        let mut multiplier = 1000;
        for word in signal {
            let word_set: HashSet<char> = word.chars().collect();

            for i in 0..mapping.len() {
                if word_set == mapping[i] {
                    total += i * multiplier;
                    multiplier = multiplier/10;
                    break
                }
            }
        }
    }



    println!("{:?}", total);
}
