// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/4/ITP1_4_C

fn run(ab: Vec<(isize, char, isize)>) -> Vec<isize> {
    ab.into_iter()
        .filter_map(|(a, op, b)| {
            match op {
                '+' => Some(a + b),
                '-' => Some(a - b),
                '*' => Some(a * b),
                '/' => Some(a / b),
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(isize, char, isize)>, Vec<isize>);

    #[test]
    fn itp1_4_c() {
        let tests = [
            TestCase(vec![(1, '+', 2), (56, '-', 18), (13, '*', 2), (100, '/', 10), (27, '+', 81), (0, '?', 0)], vec![3, 38, 26, 10, 108]),
        ];

        for TestCase(ab, expected) in tests {
            assert_eq!(run(ab), expected);
        }
    }
}
