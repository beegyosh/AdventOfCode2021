fn main() {
    let mut input: Vec<i32> = include_str!("../input.txt")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    input.sort();
    let mid = input.len()/2;
    let mut total_fuel = 0;
    for i in &input {
        total_fuel += (*i - input[mid]).abs()
    }

    println!("{}", total_fuel);
}
