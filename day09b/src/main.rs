use std::collections::{BinaryHeap, HashSet};

fn basin_search(x: usize, y: usize, heightmap: &Vec<Vec<i32>>) -> (i32, HashSet<(usize, usize)>) {
    let mut q: Vec<(usize, usize)> = vec![(x, y)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut size = 0;

    visited.insert((x, y));
    while !q.is_empty() {
        let coords = q.pop().unwrap();
        let c_x = coords.0;
        let c_y = coords.1;
        let mut neighbors: Vec<(usize, usize)> = Vec::new();


        if heightmap[c_y][c_x] != 9 {
            size += 1;

            if c_y as i32 - 1 >= 0 {
                neighbors.push((c_x, c_y - 1));
            }
            if c_y + 1 < heightmap.len() {
                neighbors.push((c_x, c_y + 1));
            }
            if c_x as i32 - 1 >= 0 {
                neighbors.push((c_x - 1, c_y));
            }
            if c_x + 1 < heightmap[c_y].len() {
                neighbors.push((c_x + 1, c_y));
            }
        }

        for i in neighbors {
            if !visited.contains(&i) {
                q.push(i);
                visited.insert(i);
            }
        }
    }
    return (size, visited)
}

fn main() {
    let input: Vec<Vec<i32>> = include_str!("../input.txt")
        .lines().map(|x| x.chars().map(|x| x as i32 - 0x30).collect()).collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(input.len() * input[0].len());
    let mut basins: BinaryHeap<i32> = BinaryHeap::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if !visited.contains(&(x, y)) {
                let (basin_size, basin_points) = basin_search(x, y, &input);
                for point in basin_points {
                    visited.insert(point);
                }
                basins.push(basin_size);
            }
        }
    }

    let result = basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap();

    println!("{:?}", result);
}
