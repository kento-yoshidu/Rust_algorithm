// https://atcoder.jp/contests/abc250/tasks/abc250_a

fn run(h: usize, w: usize, r: usize, c: usize) -> usize {
    let mut ans = 0;

    if h != 1 {
        ans += 1;
    }
    if r != 1 && r != h {
        ans += 1;
    }

    if w != 1 {
        ans += 1;
    }
    if c != 1 && c != w {
        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn abc250_a() {
        let tests = [
            TestCase(3, 4, 2, 2, 4),
            TestCase(3, 4, 1, 3, 3),
            TestCase(3, 4, 3, 4, 2),
            TestCase(1, 1, 1, 1, 0),
        ];

        for TestCase(h, w, r, c, expected) in tests {
            assert_eq!(run(h, w, r, c), expected);
        }
    }
}
