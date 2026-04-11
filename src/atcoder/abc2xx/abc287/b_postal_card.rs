// https://atcoder.jp/contests/abc287/tasks/abc287_b

fn run(_n: usize, _m: usize, s: Vec<&str>, t: Vec<&str>) -> usize {
    let s: Vec<&str> = s.into_iter().map(|str| &str[3..]).collect();

    s.into_iter()
        .filter(|str| {
            t.iter()
                .any(|str2| {
                    str == str2
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>, usize);

    #[test]
    fn abc287_b() {
        let tests = [
            TestCase(3, 3, vec!["142857", "004159", "071028"], vec!["159", "287", "857"], 2),
            TestCase(5, 4, vec!["235983", "109467", "823476", "592801", "000333"], vec!["333", "108", "467", "983"], 3),
            TestCase(4, 4, vec!["000000", "123456", "987111", "000000"], vec!["000", "111", "999", "111"], 3),
        ];

        for TestCase(n, m, s, t, expected) in tests {
            assert_eq!(run(n, m, s, t), expected);
        }
    }
}
