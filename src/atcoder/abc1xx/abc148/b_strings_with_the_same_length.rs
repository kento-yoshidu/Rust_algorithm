// https://atcoder.jp/contests/abc148/tasks/abc148_b

fn run(n: usize, s1: &str, s2: &str) -> String {
    let mut result = String::from("");

    for i in 0..n {
        result += format!("{}{}", s1.chars().nth(i).unwrap(), s2.chars().nth(i).unwrap()).as_str();
    }

    result
}

fn run2(_n: usize, s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| format!("{}{}", c1, c2))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str, &'static str);

    #[test]
    fn abc148_b() {
        let tests = [
            TestCase(2, "ip", "cc", "icpc"),
            TestCase(8, "hmhmnknk", "uuuuuuuu","humuhumunukunuku"),
            TestCase(5, "aaaaa", "aaaaa", "aaaaaaaaaa"),
        ];

        for TestCase(n, s1, s2, expected) in tests {
            assert_eq!(run(n, s1, s2), expected);
            assert_eq!(run2(n, s1, s2), expected);
        }
    }
}
