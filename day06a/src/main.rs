fn simulate_lifecycle(school: &mut Vec<u32>) {
    let mut new_fish = 0;
    for i in 0..school.len() {
        if school[i] == 0 {
            school[i] = 6;
            new_fish += 1;
        } else {
            school[i] -= 1;
        }
    }
    for _ in 0..new_fish {
        school.push(8)
    }
}

fn main() {
    let num_days = 256;
    let mut input: Vec<u32> = include_str!("../input.txt")
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    for i in 0..num_days {
        println!("{}", i.to_string());
        simulate_lifecycle(&mut input)
    }

    println!("{:?}", input.len());
}
