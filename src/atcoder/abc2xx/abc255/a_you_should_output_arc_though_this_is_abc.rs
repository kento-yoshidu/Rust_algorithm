// https://atcoder.jp/contests/abc255/tasks/abc255_a

fn run(r: usize, c: usize, arr: [[usize; 2]; 2]) -> usize {
    arr[r-1][c-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, [[usize; 2]; 2], usize);

    #[test]
    fn abc255_a() {
        let tests = [
            TestCase(1, 2, [[1, 0], [0, 1]], 0),
            TestCase(2, 2, [[1, 2], [3, 4]], 4),
            TestCase(2, 1, [[90, 80], [70, 60]], 70),
        ];

        for TestCase(r, c, a, expected) in tests {
            assert_eq!(run(r, c, a), expected);
        }
    }
}
