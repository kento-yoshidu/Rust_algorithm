// https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_a

fn run(n: usize) -> &'static str {
    for i in 1..100 {
        if i*i*i == n {
            return "YES"
        }
    }

    "NO"
}

fn run2(n: usize) -> &'static str {
    if (1..=100).any(|i| {
        i * i * i == n
    }) {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, "YES"),
            TestCase(24, "NO")
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
