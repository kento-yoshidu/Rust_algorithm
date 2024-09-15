// https://atcoder.jp/contests/abc129/tasks/abc129_a

fn run(p: usize, q: usize, r: usize) -> usize {
    let mut vec = vec![p, q, r];

    vec.sort();

    vec[0] + vec[1]
}

fn run2(p: usize, q: usize, r: usize) -> usize {
    [p+q, q+r, p+r].into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 3, 4, 4),
            TestCase(3, 2, 3, 5),
        ];

        for TestCase(p, q, r, expected) in tests {
            assert_eq!(run(p, q, r), expected);
            assert_eq!(run2(p, q, r), expected);
        }
    }
}
