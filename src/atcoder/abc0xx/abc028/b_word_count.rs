// https://atcoder.jp/contests/abc028/tasks/abc028_b

fn run(n: &str) -> Vec<usize> {
    let mut ans = vec![0; 6];

    for c in n.chars() {
        ans[(c as u8 - 65) as usize] += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase("BEAF", vec![1, 1, 0, 0, 1, 1]),
            TestCase("DECADE", vec![1, 0, 1, 2, 2, 0]),
            TestCase("ABBCCCDDDDEEEEEFFFFFF", vec![1, 2, 3, 4, 5, 6]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
