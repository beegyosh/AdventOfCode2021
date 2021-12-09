fn main() {
    let input: Vec<Vec<i32>> = include_str!("../input.txt")
        .lines().map(|x| x.chars().map(|x| x as i32 - 0x30).collect()).collect();

    let mut total = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let mut neighbors: Vec<i32> = Vec::new();
            if y as i32 - 1 >= 0 {
                neighbors.push(input[y-1][x])
            }
            if y + 1 < input.len() {
                neighbors.push(input[y+1][x])
            }
            if x as i32 - 1 >= 0 {
                neighbors.push(input[y][x-1])
            }
            if x + 1 < input[y].len() {
                neighbors.push(input[y][x+1])
            }

            neighbors.sort();

            if input[y][x] < neighbors[0] {
                total += 1 + input[y][x];
                println!("{}", input[y][x])
            }
        }
    }
    println!("{:?}", total);
}
