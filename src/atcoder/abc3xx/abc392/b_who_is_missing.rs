// https://atcoder.jp/contests/abc392/tasks/abc392_b

fn run(n: usize, m: usize, a: Vec<usize>) -> (usize, Vec<usize>) {
    let vec: Vec<usize> = (1..=n)
        .filter(|i| {
            !a.contains(i)
        })
        .collect();

    if vec.len() == 0 {
        (0, Vec::new())
    } else {
        (vec.len(), vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, (usize, Vec<usize>));

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 3, vec![3, 9, 2], (7, vec![1, 4, 5, 6, 7, 8, 10])),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
