// https://atcoder.jp/contests/abc149/tasks/abc149_c

fn is_prime(x: usize) -> bool {
    for i in 2..x {
        if i*i >= x {
            break;
        }
        if x % i == 0 {
            return false;
        }
    }

    true
}

fn run(x: usize) -> usize {
    for num in x.. {
        if is_prime(num) == true {
            return num;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc149_c() {
        let tests = [
            TestCase(36, 37),
            TestCase(30, 31),
            TestCase(37, 37),
            TestCase(89, 89),
            TestCase(59, 59),
            TestCase(64993, 64997),
            TestCase(90371, 90371),
            TestCase(92227, 92227),
            TestCase(59054, 59063),
            TestCase(56588, 56591),
            TestCase(1130, 1151),
            TestCase(1328, 1361),
            TestCase(20, 23),
            TestCase(2, 2),
            TestCase(99992, 100003),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
