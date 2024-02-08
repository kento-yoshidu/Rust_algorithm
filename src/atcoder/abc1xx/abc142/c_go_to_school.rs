// https://atcoder.jp/contests/abc142/tasks/abc142_c

pub fn run(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec = vec![0; n];

    for (i, num) in a.iter().enumerate() {
        vec[num-1] = i+1;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![2, 3, 1], vec![3, 1, 2]),
            TestCase(5, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            TestCase(8, vec![8, 2, 7, 3, 4, 5, 6, 1], vec![8, 2, 4, 5, 6, 7, 3, 1]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
