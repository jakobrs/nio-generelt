use std::{collections::VecDeque, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut words = input.split_ascii_whitespace();

    macro_rules! read {
        (raw) => {
            words.next().unwrap()
        };
        ($ty:ty) => {
            words.next().unwrap().parse::<$ty>().unwrap()
        };
        () => {
            read!(_)
        };
    }

    type Coord = (usize, usize, usize);

    let w: usize = read!();
    let h: usize = read!();
    let d: usize = read!();
    let start: Coord = (read!(), read!(), read!());
    let goal: Coord = (read!(), read!(), read!());

    let mut grid = vec![vec![vec![false; w]; h]; d];

    for i in 0..d {
        for y in 0..h {
            let line = read!(raw).as_bytes();
            for x in 0..w {
                if line[x] == b'#' {
                    grid[i][y][x] = true;
                }
            }
        }
    }

    let mut visited = vec![vec![vec![false; w]; h]; d];
    visited[start.2][start.1][start.0] = true;
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut dist = None;
    while let Some((p @ (x, y, z), d)) = queue.pop_front() {
        if p == goal {
            dist = Some(d);
        }

        for (xd, yd, zd) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let x = (x as isize + xd) as usize;
            let y = (y as isize + yd) as usize;
            let z = (z as isize + zd) as usize;

            if grid[z][y][x] && !visited[z][y][x] {
                visited[z][y][x] = true;
                queue.push_back(((x, y, z), d + 1));
            }
        }
    }

    if let Some(dist) = dist {
        println!("{}", dist);
    } else {
        println!("Tunnelen kan ikke bygges!");
    }
}
