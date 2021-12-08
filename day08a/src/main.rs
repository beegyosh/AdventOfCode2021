fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .lines().map(|x| x.split(" | ").nth(1).unwrap()).collect();

    let mut total = 0;
    for i in input {
        total += i.split(" ").fold(0, |acc, x|
            if x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7 {
                println!("{:?}", x.len());
                acc + 1
            } else {
                println!("{:?}", x.len());
                acc
            }
        );
    }

        /*
        .fold(0, |acc, x|
            if x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7 {
                println!("{:?}", x.len());
                acc + 1
            } else {
                println!("{:?}", x.len());
                acc
            }
        );*/

    println!("{:?}", total);
}
