// https://atcoder.jp/contests/abc162/tasks/abc162_c

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

pub fn run(k: usize) -> usize {
    let mut ans = 0;

    for i in 1..=k {
        for j in 1..=k {
            for l in 1..=k {
                ans += gcd(gcd(i, j), l);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 9),
            TestCase(200, 10813692),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(expected, run(n));
        }
    }
}
