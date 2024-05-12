// https://atcoder.jp/contests/abc353/tasks/abc353_a

fn run(_n: usize, h: Vec<isize>) -> isize {
    h.iter()
        .enumerate()
        .find(|(_, num)| {
            h[0] < **num
        })
        .map(|(i, _)| i as isize + 1)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![3, 2 ,5 ,2], 3),
            TestCase(3, vec![4, 3, 2], -1),
            TestCase(7, vec![10, 5, 10, 2, 10, 13, 15], 6),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
