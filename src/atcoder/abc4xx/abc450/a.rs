// https://atcoder.jp/contests/abc450/tasks/abc450_a

fn run(n: usize) -> String {
    (1..=n)
        .rev()
        .map(|n| {
            if n == 1 {
                String::from("1")
            } else {
                format!("{n},")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc450_a() {
        let tests = [
            TestCase(9, "9,8,7,6,5,4,3,2,1"),
            TestCase(5, "5,4,3,2,1"),
            TestCase(1, "1"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
