// https://atcoder.jp/contests/abc319/tasks/abc319_b

fn run(n: usize) -> String {
    let mut ans = Vec::new();

    'outer: for i in 0..=n {
        for j in 1..=9 {
            if n % j == 0 {
                if i % (n / j) == 0 {
                    ans.push(std::char::from_digit(j as u32, 10).unwrap());
                    // 最小のものを出力するので、見つかった時点でiを次に進める
                    continue 'outer
                }
            }
        }

        ans.push('-');
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(12, "1-643-2-346-1"),
            TestCase(7, "17777771"),
            TestCase(1, "11"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
