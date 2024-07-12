// https://atcoder.jp/contests/arc110/tasks/arc110_a

fn gcd(m: usize, n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

pub fn run(n: usize) -> usize {
    let mut num = 1;

    for i in 2..=n {
        num = num / gcd(num, i) * i;
    }

    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 7),
            TestCase(10, 2521),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
