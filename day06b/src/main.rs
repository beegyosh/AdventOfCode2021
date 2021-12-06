use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;


fn simulate_lifecycle(school: &mut Vec<u64>) {
    let mut new_fish = school[0];
    for i in 0..school.len()-1 {
        school[i] = school[i+1];
    }
    let last_index = school.len()-1;
    school[last_index] = new_fish;
    school[6] += new_fish;
}

fn main() {
    let num_days = 256;
    let input: Vec<usize> = include_str!("../input.txt")
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut school: Vec<u64> = vec![0,0,0,0,0,0,0,0,0];

    for i in input {
        school[i] += 1
    }


    for _ in 0..num_days {
        simulate_lifecycle(&mut school)
    }



    println!("{:?}", school);
    println!("{:?}", school.iter().sum::<u64>());
}
