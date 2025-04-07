// https://atcoder.jp/contests/abc036/tasks/abc036_b

fn run(n: usize, s: Vec<&str>) -> Vec<String> {
    let mut ans = vec![vec![' '; n]; n];

    s.iter()
        .enumerate()
        .for_each(|(i, str)| {
            str.chars()
                .enumerate()
                .for_each(|(j, c)| {
                    ans[j][n - 1 - i] = c
            })
        });

    ans.iter()
        .map(|chars| {
            chars.iter()
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec!["ooxx", "xoox", "xxxx", "xxxx"], vec!["xxxo", "xxoo", "xxox", "xxxx"]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
