// https://atcoder.jp/contests/abc460/tasks/abc460_b

fn check(x1: isize, y1: isize, r1: isize, x2: isize, y2: isize, r2: isize, ) -> bool {
    let l = (r1 - r2).pow(2);
    let m = (x1 - x2).pow(2) + (y1 - y2).pow(2);
    let r = (r1 + r2).pow(2);

    l <= m && m <= r
}

fn run(_t: isize, case: Vec<(isize, isize, isize, isize, isize, isize)>) -> Vec<&'static str> {
    case.into_iter()
        .map(|case| {
            if check(case.0, case.1, case.2, case.3, case.4, case.5) {
                "Yes"
            } else {
                "No"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, Vec<(isize, isize, isize, isize, isize, isize)>, Vec<&'static str>);

    #[test]
    fn abc460_b() {
        let tests = [
            TestCase(7, vec![(0, 0, 2, 2, 3, 2), (0, 0, 2, 2, 3, 1), (1, 2, 5, 3, 2, 1), (5, 4, 2, 8, 8, 3), (2, 1, 5, 5, 1, 2), (0, 0, 1, 0, 0, 1), (0, 0, 500000000, 1, 1000000000, 500000000)], vec!["Yes", "No", "No", "Yes", "Yes", "Yes", "No"]),
        ];

        for TestCase(t, case, expected) in tests {
            assert_eq!(run(t, case), expected);
        }
    }
}