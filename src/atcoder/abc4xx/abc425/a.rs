// https://atcoder.jp/contests/abc425/tasks/abc425_a

fn run(n: isize) -> i32 {
    let mut ans = 0;

    for i in 1..=n {
        let sign = if i % 2 == 0 { 1 } else { -1 };
        ans += sign * (i.pow(3) as i32);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, i32);

    #[test]
    fn abc425_a() {
        let tests = [
            TestCase(3, -20),
            TestCase(9, -425),
            TestCase(10, 575),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
