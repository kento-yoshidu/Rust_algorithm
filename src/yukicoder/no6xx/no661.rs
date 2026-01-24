// https://yukicoder.me/problems/no/661

fn run(_n: usize, a: Vec<usize>) -> Vec<String> {
    a.into_iter()
        .map(|n| {
            if n % 8 == 0 && n % 10 == 0 {
                "ikisugi".to_string()
            } else if n % 8 == 0 {
                "iki".to_string()
            } else if n % 10 == 0 {
                "sugi".to_string()
            } else {
                (n / 3).to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<&'static str>);

    #[test]
    fn yuki_661() {
        let tests = [
            TestCase(3, vec![3, 6, 30], vec!["1", "2", "sugi"]),
            TestCase(4, vec![36, 8, 30, 160], vec!["12", "iki", "sugi", "ikisugi"]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
