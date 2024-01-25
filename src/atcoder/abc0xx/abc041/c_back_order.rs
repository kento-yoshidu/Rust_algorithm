// https://atcoder.jp/contests/abc041/tasks/abc041_c

pub fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec: Vec<(usize, &usize)> = a.iter().enumerate().collect();

    vec.sort_by(|a, b| a.1.cmp(b.1));
    vec.reverse();

    vec.iter()
        .map(|(i, _)| i+1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![140, 180, 160], vec![2, 3, 1]),
            TestCase(2, vec![1000000000, 1], vec![1, 2]),
            TestCase(8, vec![3, 1, 4, 15, 9, 2, 6, 5], vec![4, 5, 7, 8, 3, 1, 6, 2]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(expected, run(n, a));
        }
    }
}
