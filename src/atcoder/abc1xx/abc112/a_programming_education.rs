// https://atcoder.jp/contests/abc112/tasks/abc112_a

fn run(n: usize, ab: Option<Vec<usize>>) -> String {
    if n == 1 {
        String::from("Hello World")
    } else {
        ab.unwrap()
            .into_iter()
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Option<Vec<usize>>, &'static str);

    #[test]
    fn abc112_a() {
        let tests = [
            TestCase(1, None, "Hello World"),
            TestCase(2, Some(vec![3, 5]), "8"),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
