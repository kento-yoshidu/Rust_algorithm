// https://atcoder.jp/contests/abc186/tasks/abc186_c

fn run(n: usize) -> usize {
    (1..=n)
        .filter(|num| {
            !num.to_string().contains("7") && !format!("{:0o}", num).to_string().contains("7")
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc186_c() {
        let tests = [
            TestCase(20, 17),
            TestCase(100000, 30555),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
