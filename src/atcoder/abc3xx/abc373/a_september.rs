// https://atcoder.jp/contests/abc373/tasks/abc373_a

fn run(s: [&str; 12]) -> usize {
    s.into_iter()
        .enumerate()
        .filter(|(i, str)| {
            *i+1 == str.len()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([&'static str; 12], usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"], 1),
            TestCase(["ve", "inrtfa", "npccxva", "djiq", "lmbkktngaovl", "mlfiv", "fmbvcmuxuwggfq", "qgmtwxmb", "jii", "ts", "bfxrvs", "eqvy"], 2),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
