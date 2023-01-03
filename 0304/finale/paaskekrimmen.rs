use std::{
    collections::HashMap,
    io::{BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut words = input.split_ascii_whitespace();
    let mut output = BufWriter::new(std::io::stdout());

    macro_rules! read {
        (str) => {
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
    let m = read!(usize);

    let mut adj = vec![vec![false; n]; n];
    let mut name_to_index = HashMap::new();
    let mut index_to_name = vec![];

    for i in 0..n {
        let name = read!(str);
        name_to_index.insert(name, i);
        index_to_name.push(name);

        adj[i][i] = true;
    }

    for _ in 0..m {
        let a = read!(str);
        let b = read!(str);

        let a = name_to_index[a];
        let b = name_to_index[b];

        adj[a][b] = true;
        adj[b][a] = true;
    }

    let mut colors = vec![None; n];

    fn dfs(
        adj: &[Vec<bool>],
        n: usize,
        colors: &mut [Option<bool>],
        i: usize,
        color: bool,
    ) -> bool {
        if let Some(actual_color) = colors[i] {
            return actual_color == color;
        }

        colors[i] = Some(color);
        for j in 0..n {
            if adj[i][j] == false {
                if !dfs(adj, n, colors, j, !color) {
                    return false;
                }
            }
        }

        true
    }

    for i in 0..n {
        if colors[i] == None {
            if !dfs(&adj, n, &mut colors, i, false) {
                writeln!(output, "NEI").unwrap();
                return;
            }
        }
    }

    writeln!(output, "JA").unwrap();
    for i in 0..n {
        if colors[i] == Some(true) {
            writeln!(output, "{}", index_to_name[i]).unwrap();
        }
    }
}
