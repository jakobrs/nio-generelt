use std::io::Read;

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

    let n = read!(usize);
    let u = read!(usize);
    let m = read!(usize);

    let mut c = vec![vec![]; n];

    for _ in 0..m {
        let from = read!(usize);
        let to = read!(usize);
        c[from].push(to);
    }

    let mut paths_from = vec![None; n];
    paths_from[u] = Some(1);

    fn dfs(c: &[Vec<usize>], paths_from: &mut [Option<usize>], here: usize) -> usize {
        if let Some(n) = paths_from[here] {
            n
        } else {
            let n: usize = c[here].iter().map(|&child| dfs(c, paths_from, child)).sum();
            paths_from[here] = Some(n);
            n
        }
    }

    let (from, n_paths) = (0..n)
        .map(|here| (here, dfs(&c, &mut paths_from, here)))
        .max_by_key(|&(_, p)| p)
        .unwrap();

    println!("{from}");
    println!("{n_paths}");
}
