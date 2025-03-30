// https://atcoder.jp/contests/abc096/tasks/abc096_b

fn run(v: &Vec<usize>, k: usize) -> usize {
    let sum: usize = v.iter().sum();

    sum + v.iter().max().unwrap() * (2 * k -1)
}

fn run2(v: &Vec<usize>, k: usize) -> usize {
    let max = v.iter().max().unwrap();

    let rest: usize = v.iter().filter(|num| *num != max).sum();

    max * (2 * k) + rest
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![5, 3, 11], 1, 30),
            TestCase(vec![3, 3, 4], 2, 22),
        ];

        for TestCase(v, k, expected) in tests {
            assert_eq!(run(&v, k), expected);
            assert_eq!(run2(&v, k), expected);
        }
    }
}
