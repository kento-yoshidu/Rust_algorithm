// https://atcoder.jp/contests/abc056/tasks/arc070_a

pub fn run(n: usize) -> usize {
    let mut ans = 0;
    let mut sum = 0;

    for i in 1.. {
        ans += 1;
        sum += i;

        if n <= sum {
            return ans;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 3),
            TestCase(2, 2),
            TestCase(11, 5),
            TestCase(11, 5),
            TestCase(1000000000, 44721),
            TestCase(999999999, 44721),
            TestCase(1, 1),
            TestCase(999961560, 44720),
            TestCase(999961559, 44720),
            TestCase(999961561, 44721),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
