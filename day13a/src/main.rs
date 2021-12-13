use std::cmp::max;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .lines().collect();
    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut x_bound = 0;
    let mut y_bound = 0;
    let mut folds: Vec<(char, usize)> = Vec::new();

    for line in input {
        if line.starts_with("fold along y") {
            folds.push(('y', line.split("=").last().unwrap().parse().unwrap()));
        } else if line.starts_with("fold along x") {
            folds.push(('x', line.split("=").last().unwrap().parse().unwrap()));
        } else {
            let mut c_split = line.split(",");
            let x = match c_split.next().unwrap().parse::<usize>() {
                Ok(val) => val,
                Err(_) => continue
            };
            let y = match c_split.next().unwrap().parse::<usize>() {
                Ok(val) => val,
                Err(_) => continue
            };
            x_bound = max(x + 1, x_bound);
            y_bound = max(y + 1, y_bound);
            coords.push((x,y))
        }
    }

    let mut paper: Vec<Vec<usize>> = vec![vec![0; x_bound]; y_bound];

    for c in coords {
        paper[c.1][c.0] = 1;
    }

    for fold in folds {
        if fold.0 == 'y' {
            for y in fold.1..y_bound {
                for x in 0..x_bound {
                    if paper[y][x] == 1 {
                        let new_loc = fold.1 as isize - (y as isize - fold.1 as isize);
                        paper[y][x] = 0;
                        if new_loc > -1 {
                            paper[new_loc as usize][x] = 1;
                        }
                    }
                }
            }
        }

        if fold.0 == 'x' {
            for y in 0..y_bound {
                for x in fold.1..x_bound {
                    if paper[y][x] == 1 {
                        let new_loc = fold.1 as isize - (x as isize - fold.1 as isize);
                        paper[y][x] = 0;
                        if new_loc > -1 {
                            paper[y][new_loc as usize] = 1;
                        }
                    }
                }
            }
        }
        break;
    }

    let mut total = 0;

    for i in 0..paper.len() {
        for j in 0..paper[0].len() {
            if paper[i][j] == 1 {
                total += 1;
            }
            print!("{}", paper[i][j])
        }
        print!("\n")
    }

    println!("Total dots: {}", total)
}
