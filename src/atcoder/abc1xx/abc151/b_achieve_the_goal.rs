// https://atcoder.jp/contests/abc151/tasks/abc151_b

pub fn run(n: i32, k: i32, m: i32, vec: Vec<i32>) -> i32 {
    let sum: i32 = vec.iter().sum();

    let line = n * m;

    if (line - sum) > k {
        -1
    } else if (line - sum) < 0 {
        0
    } else {
        line - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, i32, Vec<i32>, i32);

    #[test]
    fn abc151_b() {
        let tests = [
            TestCase(5, 10, 7, vec![8, 10, 3, 6], 8),
            TestCase(4, 100, 60, vec![100, 100, 100], 0),
            TestCase(4, 100, 60, vec![0, 0, 0], -1),
        ];

        for TestCase(n, k, m, v, expected) in tests {
            assert_eq!(run(n, k, m, v), expected);
        }
    }
}
