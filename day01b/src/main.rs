fn main() {
    let mut count = 0;
    let input: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap()).collect();
    let mut input_windows = input.windows(3);

    let mut last_window_sum: usize = input_windows.next().unwrap().iter().sum();
    for window in input_windows {
        let window_sum = window.iter().sum();
        if window_sum > last_window_sum {
            count += 1;
        }
        last_window_sum = window_sum;
    }
    println!(
        "{:?}",
        count
    );
}
