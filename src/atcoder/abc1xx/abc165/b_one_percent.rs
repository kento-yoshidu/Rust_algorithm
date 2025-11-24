// https://atcoder.jp/contests/abc165/tasks/abc165_b

fn calc(saving: u128, input: u128, year: u128) -> u128 {
    if saving >= input {
        year
    } else {
        calc(saving * 101 / 100, input, year+1)
    }
}

fn run(input: u128) -> u128 {
    calc(100, input, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u128, u128);

    #[test]
    fn abc165_b() {
        let tests = [
            TestCase(103, 3),
            TestCase(1000000000000000000, 3760),
            TestCase(1333333333, 1706),
        ];

        for TestCase(i, expected) in tests {
            assert_eq!(run(i), expected);
        }
    }
}
