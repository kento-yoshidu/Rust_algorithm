// https://atcoder.jp/contests/abc131/tasks/abc131_c

fn gcd(c: usize, d: usize) -> usize {
    if d == 0 {
        c
    } else {
        gcd(d, c % d)
    }
}

fn lcm(c: usize, d: usize) -> usize {
    c / gcd(c, d) * d
}

fn calc(num: usize, c: usize, d: usize) -> usize {
    let c_count = num / c;
    let d_count = num / d;
    let l = lcm(c, d);

    num - c_count - d_count + num / l
}

fn run(a: usize, b: usize, c: usize, d: usize) -> usize {
    calc(b, c, d) - calc(a-1, c, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn abc131_c() {
        let tests = [
            TestCase(4, 9, 2, 3, 2),
            TestCase(10, 40, 6, 8, 23),
            TestCase(314159265358979323, 846264338327950288, 419716939, 937510582, 532105071133627368),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
