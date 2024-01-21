// https://atcoder.jp/contests/abc337/tasks/abc337_a

pub fn run(_n: usize, xy: Vec<(usize, usize)>) -> &'static str {
    let mut t = 0;
    let mut a = 0;

    for (x, y) in xy {
        t += x;
        a += y;
    }

    if t > a {
        "Takahashi"
    } else if a > t {
        "Aoki"
    } else {
        "Draw"
    }
}

pub fn run2(_n: usize, xy: Vec<(usize, usize)>) -> &'static str {
    let (t, a) = xy.iter()
        .fold((0, 0), |state, (x, y)| {
            (state.0+x, state.1+y)
        });

    if t > a {
        "Takahashi"
    } else if a > t {
        "Aoki"
    } else {
        "Draw"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(10, 2), (10, 1), (10, 2), (3, 2)], "Takahashi"),
            TestCase(6, vec![(5, 4), (4, 5), (2, 4), (1, 6), (7, 1), (3, 2)], "Draw"),
            TestCase(4, vec![(0, 0), (10, 10), (50, 50), (0, 100)], "Aoki"),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(expected, run(n, xy.clone()));
            assert_eq!(expected, run2(n, xy));
        }
    }
}