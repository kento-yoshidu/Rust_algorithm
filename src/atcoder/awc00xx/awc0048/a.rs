// https://atcoder.jp/contests/awc0048/tasks/awc0048_a

fn run(_n: usize, sk: Vec<(&str, usize)>) -> Vec<&'static str> {
    sk.into_iter()
        .map(|(s, k)| {
            let even = k % 2 == 0;

            match (s, even) {
                ("Yes", true) => "Yes",
                ("Yes", false) => "No",
                ("No", true) => "No",
                ("No", false) => "Yes",
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, Vec<&'static str>);

    #[test]
    fn awc0048_a() {
        let tests = [
            TestCase(3, vec![("Yes", 1), ("No", 2), ("Yes", 3)], vec!["No", "No", "No"]),
            TestCase(4, vec![("Yes", 2), ("No", 1), ("No", 4), ("Yes", 5)], vec!["Yes", "Yes", "No", "No"]),
            TestCase(6, vec![("Yes", 100), ("No", 99), ("Yes", 1000000), ("No", 999999), ("Yes", 42), ("No", 7)], vec!["Yes", "Yes", "Yes", "Yes", "Yes", "Yes"]),
            TestCase(10, vec![("Yes", 1000000000), ("No", 1000000000), ("Yes", 999999999), ("No", 999999999), ("Yes", 1), ("No", 1), ("Yes", 2), ("No", 2), ("Yes", 500000000), ("No", 500000001)], vec!["Yes", "No", "No", "Yes", "No", "Yes", "Yes", "No", "Yes", "Yes"]),
            TestCase(1, vec![("No", 1)], vec!["Yes"]),
        ];

        for TestCase(n, sk, expected) in tests {
            assert_eq!(run(n, sk), expected);
        }
    }
}
