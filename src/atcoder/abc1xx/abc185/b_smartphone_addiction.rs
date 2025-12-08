// https://atcoder.jp/contests/abc185/tasks/abc185_b

fn run(n: isize, _m: isize, t: isize, ab: Vec<(isize, isize)>) -> &'static str {
    let mut rest = n;
    let mut start = 0;

    for (a, b) in ab {
        if rest - (a - start) <= 0 {
            return "No";
        };

        rest -= a - start;

        if rest + (b - a) > n {
            rest = n;
        } else {
            rest += b - a;
        }

        start = b;
    }

    if rest <= t - start {
        return "No";
    } else {
        return "Yes";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, Vec<(isize, isize)>, &'static str);

    #[test]
    fn abc185_b() {
        let tests = [
            TestCase(10, 2, 20, vec![(9, 11), (13, 17)], "Yes"),
            TestCase(10, 2, 20, vec![(9, 11), (13, 16)], "No"),
            TestCase(15, 3, 30, vec![(5, 8), (15, 17), (24, 27)], "Yes"),
            TestCase(20, 1, 30, vec![(20, 29)], "No"),
            TestCase(20, 1, 30, vec![(1, 10)], "No"),
        ];

        for TestCase(n, m, t, ab, expected) in tests {
            assert_eq!(run(n, m, t, ab), expected);
        }
    }
}
