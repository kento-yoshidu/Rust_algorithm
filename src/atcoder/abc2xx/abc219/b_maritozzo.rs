// https://atcoder.jp/contests/abc219/tasks/abc219_b

fn run(s: Vec<&str>, t: &str) -> String {
    let nums: Vec<usize> =
        t.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    nums.into_iter()
        .map(|i| {
            s[i-1]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, &'static str, &'static str);

    #[test]
    fn abc219_b() {
        let tests = [
            TestCase(vec!["mari", "to", "zzo"], "1321", "marizzotomari"),
            TestCase(vec!["abra", "cad", "abra"], "123", "abracadabra"),
            TestCase(vec!["a", "b", "c"], "1", "a"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
