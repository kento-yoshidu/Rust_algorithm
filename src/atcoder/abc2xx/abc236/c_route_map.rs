// https://atcoder.jp/contests/abc236/tasks/abc236_c

fn run(_n: usize, _m: usize, s: Vec<&str>, t: Vec<&str>) -> Vec<&'static str> {
    s.into_iter()
        .scan(0, |i, station| {
            if station == t[*i] {
                *i += 1;
                Some("Yes")
            } else {
                Some("No")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc236_c() {
        let tests = [
            TestCase(5, 3, vec!["tokyo", "kanda", "akiba", "okachi", "ueno"], vec!["tokyo", "akiba", "ueno"], vec!["Yes", "No", "Yes", "No", "Yes"]),
            TestCase(7, 7, vec!["a", "t", "c", "o", "d", "e", "r"], vec!["a", "t", "c", "o", "d", "e", "r"], vec!["Yes", "Yes", "Yes", "Yes", "Yes", "Yes", "Yes"]),
        ];

        for TestCase(n, m, s, t, expected) in tests {
            assert_eq!(run(n, m, s, t), expected);
        }
    }
}
