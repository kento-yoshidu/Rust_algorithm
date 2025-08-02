// https://atcoder.jp/contests/abc127/tasks/abc127_a

fn run(age: usize, price: usize) -> usize {
    if 12 < age {
        price
    } else if 6 <= age {
        price / 2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc127_a() {
        let tests = [
            TestCase(30, 100, 100),
            TestCase(12, 100, 50),
            TestCase(0, 100, 0),
        ];

        for TestCase(age, price, expected) in tests {
            assert_eq!(run(age, price), expected);
        }
    }
}
