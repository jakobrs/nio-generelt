use std::{cell::Cell, collections::VecDeque, fmt::Write, io::Read, rc::Rc};

struct Paper {
    visited: Cell<bool>,
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

    let mut authors: Vec<Vec<Rc<Paper>>> = vec![vec![]; n];

    for _ in 0..m {
        let k = read!(usize);
        let v = words
            .by_ref()
            .take(k)
            .map(|i| i.parse::<usize>().unwrap() - 1)
            .collect();

        let paper = Rc::new(Paper {
            visited: Cell::new(false),
            authors: v,
        });

        for &author in &paper.authors {
            authors[author].push(paper.clone());
        }
    }

    let mut dists = vec![usize::MAX; n];
    let mut queue = VecDeque::new();
    dists[0] = 0;
    queue.push_back(0);

    while let Some(author) = queue.pop_front() {
        let dist = dists[author];

        for paper in &authors[author] {
            if !paper.visited.replace(true) {
                for &coauthor in &paper.authors {
                    if dists[coauthor] == usize::MAX {
                        dists[coauthor] = dist + 1;

                        queue.push_back(coauthor);
                    }
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
