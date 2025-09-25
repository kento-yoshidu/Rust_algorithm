// https://atcoder.jp/contests/abc131/tasks/abc131_a

fn run(num: usize) -> &'static str {
    let a = num / 1000;
    let b = num / 100 % 10;
    let c = num / 10 % 10;
    let d = num % 10;

    if a == b || b == c || c == d {
        "Bad"
    } else {
        "Good"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCaes(usize, &'static str);

    #[test]
    fn abc131_a() {
        let tests = [
            TestCaes(3776, "Bad"),
            TestCaes(8080, "Good"),
            TestCaes(1333, "Bad"),
            TestCaes(0024, "Bad"),
        ];

        for TestCaes(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
