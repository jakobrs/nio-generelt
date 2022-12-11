use std::{collections::VecDeque, fmt::Write, io::Read};

struct Paper {
    authors: Vec<usize>,
}

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
    let m = read!(usize);

    let mut papers = vec![];

    let mut authors: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        let k = read!(usize);
        let v = words
            .by_ref()
            .take(k)
            .map(|i| i.parse::<usize>().unwrap() - 1)
            .collect();

        let i = papers.len();
        for &author in &v {
            let a: &mut Vec<usize> = &mut authors[author];
            a.push(i);
        }

        papers.push(Paper { authors: v });
    }

    let mut dists = vec![usize::MAX; n];
    let mut queue = VecDeque::new();
    dists[0] = 0;
    queue.push_back(0);

    while let Some(author) = queue.pop_front() {
        let dist = dists[author];

        for &paper_id in &authors[author] {
            for coauthor in papers[paper_id].authors.drain(..) {
                if dists[coauthor] == usize::MAX {
                    dists[coauthor] = dist + 1;

                    queue.push_back(coauthor);
                }
            }
        }
    }

    let mut output = String::new();
    for dist in dists {
        writeln!(output, "{}", dist as isize).unwrap();
    }

    print!("{output}");
}
