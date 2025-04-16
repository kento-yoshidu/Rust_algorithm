// https://atcoder.jp/contests/joi2021yo1c/tasks/joi2021_yo1c_a

fn run(a: isize, b: isize) -> (isize, isize) {
    let plus = a + b;
    let minus = a - b;

    if plus > minus {
        (plus, minus)
    } else {
        (minus, plus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, (isize, isize));

    #[test]
    fn test() {
        let tests = [
            TestCase(-2, 1, (-1, -3)),
            TestCase(-3, -4, (1, -7)),
            TestCase(5, 0, (5, 5)),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
