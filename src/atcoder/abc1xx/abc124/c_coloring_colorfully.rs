// https://atcoder.jp/contests/abc124/tasks/abc124_c

fn run(s: &str) -> usize {
    let ans = s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .fold([0, 0], |[l, r], (i, num)| {
            if i % 2 == num as usize {
                [l + 1, r]
            } else {
                [l, r + 1]
            }
        });

    ans.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc124_c() {
        let tests = [
            TestCase("000", 1),
            TestCase("10010010", 3),
            TestCase("0101010101010101", 0),
            TestCase("1010101010101010", 0),
            TestCase("0", 0),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
