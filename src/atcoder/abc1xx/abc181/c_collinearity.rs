// https://atcoder.jp/contests/abc181/tasks/abc181_c

fn run(n: usize, xy: Vec<(isize, isize)>) -> &'static str {
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                let (x1, y1) = xy[i];
                let (x2, y2) = xy[j];
                let (x3, y3) = xy[k];

                if (y1-y2)*(x1-x3) == (y1-y3)*(x1-x2) {
                    return "Yes";
                }
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize)>, &'static str);

    #[test]
    fn abc181_c() {
        let tests = [
            TestCase(4, vec![(0, 1), (0, 2), (0, 3), (1, 1)], "Yes"),
            TestCase(14, vec![(5, 5), (0, 1), (2, 5), (8, 0), (2, 1), (0, 0), (3, 6), (8, 6), (5, 9), (7, 9), (3, 4), (9, 2), (9, 8), (7, 2)], "No"),
            TestCase(9, vec![(8, 2), (2, 3), (1, 3), (3, 7), (1, 0), (8, 8), (5, 6), (9, 7), (0, 1)], "Yes"),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
