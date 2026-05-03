// https://atcoder.jp/contests/abc304/tasks/abc304_b

fn run(n: usize) -> usize {
    // 桁数を求める
    let mut len = n.to_string().len();

    if len <= 3 {
        return n;
    } else {
        len -= 3;
    }

    let tmp = 10_i32.pow(len as u32);

    (n / tmp as usize) * tmp as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc304_b() {
        let tests = [
            TestCase(20230603, 20200000),
            TestCase(0, 0),
            TestCase(304, 304),
            TestCase(500600, 500000),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
