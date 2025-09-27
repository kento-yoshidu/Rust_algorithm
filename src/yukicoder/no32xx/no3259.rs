// https://yukicoder.me/problems/no/3259

fn run(l: usize, r: usize) -> usize {
    let mut ans = 0;

    if l <= 295 && 296 <= r {
        ans += 1;
    }

    if l <= 416 && 417 <= r {
        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn yuki_3259() {
        let tests = [
            TestCase(245, 420, 2),
            TestCase(417, 417, 0),
            TestCase(290, 300, 1),
        ];

        for TestCase(l, r, expected) in tests {
            assert_eq!(run(l, r), expected);
        }
    }
}
