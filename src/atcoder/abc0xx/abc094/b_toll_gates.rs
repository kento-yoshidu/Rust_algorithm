// https://atcoder.jp/contests/abc094/tasks/abc094_b

fn run(n: usize, _m: usize, x: usize, a: Vec<usize>) -> usize {
    let to_start = (1..x).filter(|num| {
        a.contains(num)
    }).count();

    let to_end = (x+1..=n).filter(|num| {
        a.contains(num)
    }).count();

    to_start.min(to_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, 3, vec![1, 2, 4], 1),
            TestCase(7, 3, 2, vec![4, 5, 6], 0),
            TestCase(10, 7, 5, vec![1, 2, 3, 4, 6, 8, 9], 3),
        ];

        for TestCase(n, m, x, a, expected) in tests {
            assert_eq!(run(n, m, x, a), expected);
        }
    }
}
