use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut words = input.split_ascii_whitespace();

    macro_rules! read {
        ($ty:ty) => {
            words.next().unwrap().parse::<$ty>().unwrap()
        };
    }

    let n = read!(usize);

    let mut columns = vec![vec![]; 30_001];

    for i in 0..n {
        let x = read!(i64);
        let y = read!(i64);

        let x_idx = (x + 15_000) as usize;
        let y_idx = (y + 15_000) as usize;
        columns[x_idx].push((i + 1, y_idx));
    }

    for column in &mut columns {
        column.sort_by_key(|&(_i, y)| y);
    }

    let (_k, i, j) = solve(&columns);

    println!("{i} {j}");
}

// columns[x] contains elements of the form (index, y)
// returns a triple (distance, first index, second index)
fn solve(columns: &[Vec<(usize, usize)>]) -> (usize, usize, usize) {
    if columns.len() == 1 {
        match columns[0]
            .windows(2)
            .map(|it| {
                let i = it[0].0;
                let j = it[1].0;
                (it[1].1 - it[0].1, i.min(j), i.max(j))
            })
            .min()
        {
            Some((n, i, j)) => (n.pow(2), i, j),
            None => (usize::MAX / 3, 0, 0),
        }
    } else {
        let mid = columns.len() / 2;
        let left = solve(&columns[..mid]);
        let right = solve(&columns[mid..]);

        let mut best = left.min(right);

        let dist = (best.0 as f64).sqrt() as usize;

        let lower_bound = mid.saturating_sub(dist);
        let upper_bound = (mid + dist).min(columns.len());

        let slice = &columns[lower_bound..upper_bound];

        let mut combined: Vec<_> = slice
            .into_iter()
            .enumerate()
            .map(|(x, v)| v.iter().map(move |&(i, y)| (x, y, i)))
            .flatten()
            .collect();

        combined.sort_by_key(|&(_x, y, _i)| y);

        for i in 0..combined.len() {
            for d in 1..=10 {
                let j = i + d;
                if j < combined.len() {
                    let p0 = combined[i];
                    let p1 = combined[j];

                    let new_k = ((p0.0 as i64 - p1.0 as i64).pow(2)
                        + (p0.1 as i64 - p1.1 as i64).pow(2))
                        as usize;

                    let idx_i = p0.2;
                    let idx_j = p1.2;
                    let this = (new_k, idx_i.min(idx_j), idx_i.max(idx_j));

                    best = best.min(this);
                }
            }
        }

        best
    }
}
