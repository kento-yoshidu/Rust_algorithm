// https://atcoder.jp/contests/awc0039/tasks/awc0039_a

fn run(_n: usize, _m: usize, _q: usize, pc: Vec<(usize, char)>, t: Vec<&str>) -> Vec<&'static str> {
    t.into_iter()
        .map(|str| {
            if pc.iter()
                .all(|(p, c)| {
                    &str.chars().nth(*p-1).unwrap() == c
                }) {
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

    struct TestCase(usize, usize, usize, Vec<(usize, char)>, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn awc0039_a() {
        let tests = [
            TestCase(5, 2, 3, vec![(1, 'a'), (4, 'd')], vec!["abcde", "xbcde", "abcdz"], vec!["Yes", "No", "Yes"]),
            TestCase(3, 0, 2, vec![], vec!["abc", "xyz"], vec!["Yes", "Yes"]),
            TestCase(10, 4, 5, vec![(2, 'b'), (5, 'e'), (7, 'g'), (10, 'z')], vec!["abcdefghiz", "xbxdexgxxz", "abcdeagxyz", "xbxdexhxxz", "abcdefghij"], vec!["Yes", "Yes", "Yes", "No", "No"]),
        ];

        for TestCase(n, m, q, pc, t, expected) in tests {
            assert_eq!(run(n, m, q, pc, t), expected);
        }
    }
}
