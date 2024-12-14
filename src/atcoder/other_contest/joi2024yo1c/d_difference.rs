// https://atcoder.jp/contests/joi2024yo1c/tasks/joi2024_yo1c_d

fn run(k: usize, n: usize, a: Vec<usize>, m: usize, b: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in a.iter() {
        for j in b.iter() {
            if i + k == *j {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 4, vec![1, 8, 6, 8], 3, vec![7, 9, 4], 3),
            TestCase(66, 4, vec![31, 41, 59, 26], 5, vec![29, 97, 92, 45, 8], 2),
            TestCase(99, 5, vec![1, 1, 1, 1, 1], 6, vec![100, 100, 100, 100, 100, 100], 50),
            TestCase(100, 1, vec![11], 1, vec![18], 0),
        ];

        for TestCase(k, n, a, m, b, expected) in tests {
            assert_eq!(run(k ,n ,a ,m, b), expected);
        }
    }
}
