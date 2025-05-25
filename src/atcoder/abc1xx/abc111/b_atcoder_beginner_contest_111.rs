// https://atcoder.jp/contests/abc111/tasks/abc111_b

fn run(n: u32) -> u32 {
    for i in n..=999 {
        if i/100 == i%100/10 && i/100 == i%10 {
            return i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, u32);

    #[test]
    fn abc111_b() {
        let tests = [
            TestCase(111, 111),
            TestCase(112, 222),
            TestCase(750, 777),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
