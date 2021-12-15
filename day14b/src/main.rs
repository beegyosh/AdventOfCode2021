use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .lines().collect();

    let template: Vec<char> = input[0].chars().collect();
    let mut rules: Vec<(&str, char)> = Vec::new();
    for rule_string in input[2..].iter() {
        let rule: Vec<&str> = rule_string.split(" -> ").collect();
        rules.push((rule[0], rule[1].chars().next().unwrap()));
    }

    let mut pairs: HashMap<String, usize> = HashMap::new();
    for i in 0..template.len()-1 {
        let mut p = String::from("");
        p.push(template[i]);
        p.push(template[i+1]);
        *pairs.entry(p).or_insert(0) += 1;
    }

    let mut counts_map: HashMap<char, usize> = HashMap::new();
    for c in template {
        *counts_map.entry(c).or_insert(0) += 1;
    }

    let steps = 40;

    for _ in 1..steps+1 {

        let mut insertions: Vec<(String, usize)> = Vec::new();
        for rule in &rules {
            let p = *pairs.entry(rule.0.to_string()).or_default();
            if *pairs.get(&rule.0.to_string()).unwrap() > 0 {
                *counts_map.entry(rule.1).or_default() += p;
                let old_pair: Vec<char> = rule.0.chars().collect();
                let mut left_pair = String::from(old_pair[0]);
                left_pair.push(rule.1);
                let mut right_pair = String::from(rule.1);
                right_pair.push(old_pair[1]);
                insertions.push((left_pair, p));
                insertions.push((right_pair, p));
                *pairs.entry(rule.0.to_string()).or_default() = 0;
            }
        }

        for i in insertions {
            *pairs.entry(i.0).or_default() += i.1
        }
    }

    let most_common = counts_map.iter().max_by_key(|&(_, x)| x).unwrap().1;
    let least_common = counts_map.iter().min_by_key(|&(_, x)| x).unwrap().1;

    println!("{:?}", most_common-least_common);
}
