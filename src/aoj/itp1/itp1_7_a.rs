// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/7/ITP1_7_A

fn run(mfr: Vec<(isize, isize, isize)>) -> Vec<char> {
    mfr.into_iter()
        .filter_map(|(m, f, r)| {
            if m == -1 && f == -1 && r == -1 {
                None
            } else if m == -1 || f == -1 {
                Some('F')
            } else if m + f >= 80 {
                Some('A')
            } else if m + f >= 65 {
                Some('B')
            } else if m + f >= 50 {
                Some('C')
            } else if m + f >= 30 {
                if r >= 50 {
                    Some('C')
                } else {
                    Some('D')
                }
            } else {
                Some('F')
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(isize, isize, isize)>, Vec<char>);

    #[test]
    fn itp1_7_a() {
        let tests = [
            TestCase(vec![(40, 42, -1), (20, 30, -1), (0, 2, -1), (-1, -1, -1)], vec!['A', 'C', 'F']),
        ];

        for TestCase(mfr, expected) in tests {
            assert_eq!(run(mfr), expected);
        }
    }
}
