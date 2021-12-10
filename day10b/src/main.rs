use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .lines().collect();

    let mut scores: Vec<i64> = Vec::new();

    for line in input {
        let mut symbols: Vec<char> = Vec::new();
        let mut error = false;
        let mut score: i64 = 0;
        for i in line.chars() {
            if i == '(' || i == '{' || i == '<' || i == '[' {
                symbols.push(i)
            } else if i == ')' {
                if *symbols.last().unwrap() != '(' {
                    error = true;
                    break
                }
                symbols.pop();
            } else if i == '}' {
                if *symbols.last().unwrap() != '{' {
                    error = true;
                    break
                }
                symbols.pop();
            } else if i == '>' {
                if *symbols.last().unwrap() != '<' {
                    error = true;
                    break
                }
                symbols.pop();
            } else if i == ']' {
                if *symbols.last().unwrap() != '[' {
                    error = true;
                    break
                }
                symbols.pop();
            } else {
                panic!("Found a weird symbol: {}", i)
            }
        }
        if error == false {
            while !symbols.is_empty() {
                let n = symbols.pop().unwrap();
                score *= 5;
                if n == '(' {
                    score += 1;
                }
                if n == '[' {
                    score += 2;
                }
                if n == '{' {
                    score += 3;
                }
                if n == '<' {
                    score += 4;
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{:?}", scores[scores.len()/2])
}
