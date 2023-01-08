use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut words = input.split_ascii_whitespace();

    macro_rules! read {
        (raw) => {
            words.next().unwrap()
        };
        (iter $ty:ty) => {
            words.by_ref().map(|s| s.parse::<$ty>().unwrap())
        };
        (iter $ty:ty; $n:expr) => {
            words.by_ref().take($n).map(|s| s.parse::<$ty>().unwrap())
        };
        ($ty:ty) => {
            words.next().unwrap().parse::<$ty>().unwrap()
        };
        ($ty:ty; $n:expr) => {
            words
                .by_ref()
                .take($n)
                .map(|s| s.parse::<$ty>().unwrap())
                .collect::<Vec<_>>()
        };
        () => {
            read!(_)
        };
    }

    let n = read!(usize);
    let t = read!(usize);

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    enum PointKind {
        House,
        Plot,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct Point {
        position: i64,
        kind: PointKind,
        nearest_house: u64,
    }

    let mut points = vec![];
    points.extend(read!(iter i64; n).map(|x| Point {
        position: x,
        kind: PointKind::House,
        nearest_house: u64::MAX,
    }));
    points.extend(read!(iter i64; t).map(|x| Point {
        position: x,
        kind: PointKind::Plot,
        nearest_house: u64::MAX,
    }));
    points.sort();

    // LTR pass
    let mut prev_house = i64::MIN;
    for point in &mut points {
        if point.kind == PointKind::House {
            prev_house = point.position;
        }

        point.nearest_house = point.position.abs_diff(prev_house);
    }

    // LTR pass
    let mut prev_house = i64::MAX;
    for point in points.iter_mut().rev() {
        if point.kind == PointKind::House {
            prev_house = point.position;
        }

        point.nearest_house = point.nearest_house.min(point.position.abs_diff(prev_house));
    }

    // Output pass
    for point in points {
        if point.kind == PointKind::Plot {
            println!("{}", point.nearest_house);
        }
    }
}
