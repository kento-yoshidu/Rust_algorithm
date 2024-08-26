// https://atcoder.jp/contests/abc297/tasks/abc297_a

pub fn run(n: usize, d: isize, t: Vec<isize>) -> isize {
    for i in 0..n-1 {
        if t[i as usize + 1] - t[i as usize] <= d {
            return t[i as usize + 1];
        }
    }

    -1
}

pub fn run2(n: usize, d: isize, t: Vec<isize>) -> isize {
    (0..n-1)
        .find(|i| {
            t[i+1] - t[*i] <= d
        })
        .map(|i| t[i+1])
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 500, vec![300, 900, 1300, 1700], 1300),
            TestCase(5, 99, vec![100, 200, 300, 400, 500], -1),
            TestCase(4, 500, vec![100, 600, 1100, 1600], 600),
        ];

        for TestCase(n, d, t, expected) in tests {
            assert_eq!(run(n, d, t.clone()), expected);
            assert_eq!(run2(n, d, t.clone()), expected);
        }
    }
}
