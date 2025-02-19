// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ak

fn run(n: usize, m: usize, b: usize, a: Vec<usize>, c: Vec<usize>) -> usize {
    let a_sum = a.into_iter().sum::<usize>() * m;
    let c_sum = c.into_iter().sum::<usize>() * n;

    a_sum + c_sum + b * n * m
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 100, vec![10, 20], vec![1, 2, 3], 702),
        ];

        for TestCase(n, m, b, a, c, expected) in tests {
            assert_eq!(run(n, m, b, a, c), expected);
        }
    }
}
