use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .lines().collect();

    let mut total = 0;
    let mut errors: HashMap<char, usize> = HashMap::new();
    for c in ")}>]".chars() { errors.insert(c, 0); }

    for line_i in 0..input.len() {
        let mut symbols: Vec<char> = Vec::new();
        let line = input[line_i];
        for i in line.chars() {
            if i == '(' || i == '{' || i == '<' || i == '[' {
                symbols.push(i)
            } else if i == ')' {
                if *symbols.last().unwrap() != '(' {
                    *errors.get_mut(&')').unwrap() += 1;
                    break
                }
                symbols.pop();
            } else if i == '}' {
                if *symbols.last().unwrap() != '{' {
                    *errors.get_mut(&'}').unwrap() += 1;
                    break
                }
                symbols.pop();
            } else if i == '>' {
                if *symbols.last().unwrap() != '<' {
                    *errors.get_mut(&'>').unwrap() += 1;
                    break
                }
                symbols.pop();
            } else if i == ']' {
                if *symbols.last().unwrap() != '[' {
                    *errors.get_mut(&']').unwrap() += 1;
                    break
                }
                symbols.pop();
            } else {
                panic!("Found a weird symbol: {}", i)
            }
        }
    }

    for (symbol, count) in errors.iter() {
        if *symbol == ')' {
            total += count * 3
        }
        if *symbol == ']' {
            total += count * 57
        }
        if *symbol == '}' {
            total += count * 1197
        }
        if *symbol == '>' {
            total += count * 25137
        }
    }

    println!("{:?}", total)
}
