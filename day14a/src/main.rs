use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .lines().collect();

    let mut template: Vec<char> = input[0].chars().collect();
    let mut rules: HashMap<&str, char> = HashMap::new();
    for rule_string in input[2..].iter() {
        let rule: Vec<&str> = rule_string.split(" -> ").collect();
        rules.insert(rule[0], rule[1].chars().next().unwrap());
    }

    let steps = 10;
    let mut output: Vec<char> = Vec::new();
    for _ in 0..steps {
        output.clear();
        for i in 0..template.len() {
            output.push(template[i]);
            if i != template.len() - 1 {
                let mut pair = String::from("");
                pair.push(template[i]);
                pair.push(template[i + 1]);
                if rules.contains_key(pair.as_str()) {
                    output.push(rules[pair.as_str()])
                }
            }
        }
        template.clear();
        template = output.iter().map(|x| *x).collect();
    }

    let mut counts_map: HashMap<char, usize> = HashMap::new();
    for i in output.iter().map(|x| *x) {
        *counts_map.entry(i).or_insert(0) += 1;
    }

    let most_common = counts_map.iter().max_by_key(|&(_, x)| x).unwrap().1;
    let least_common = counts_map.iter().min_by_key(|&(_, x)| x).unwrap().1;

    println!("{:?}", most_common-least_common);
}
