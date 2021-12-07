fn triangulate(x: i32) -> i32 {
    return (x * (x + 1)) / 2
}

fn main() {
    let mut input: Vec<i32> = include_str!("../input.txt")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    input.sort();
    let avg: i32 = input.iter().sum::<i32>() / input.len() as i32;

    let mut best_fuel = triangulate(input[input.len()-1] - input[0]) * input.len() as i32;
    let mut best_m = -1;
    for m in input[0]..input[input.len()-1] {
        let mut total_fuel = 0;
        for i in &input {
            let mut diff: i32 = (*i - m);
            diff = diff.abs();
            total_fuel += triangulate(diff);
        }
        if total_fuel < best_fuel {
            best_fuel = total_fuel;
            best_m = m
        }
    }

    println!("{} {} {}", best_fuel, best_m, avg);
}