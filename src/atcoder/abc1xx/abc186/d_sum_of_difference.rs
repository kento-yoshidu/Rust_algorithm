// https://atcoder.jp/contests/abc186/tasks/abc186_d

fn run(n: usize, a: Vec<isize>) -> isize {
    let mut vec = a.clone();

    vec.sort();

    let mut cum = Vec::new();
    cum.push(vec[0]);

    for i in 1..n {
        cum.push(cum[i-1] + vec[i]);
    }

    (0..n)
        .map(|i| vec[i] * (i+1) as isize - cum[i])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![5, 1, 2], 8),
            TestCase(5, vec![31, 41, 59, 26, 53], 176),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
