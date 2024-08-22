// https://atcoder.jp/contests/abc339/tasks/abc339_c

fn run(n: usize, a: Vec<isize>) -> isize {
    let mut cum = vec![0];

    for i in 0..n {
        cum.push(a[i] + cum[i]);
    }

    let min = cum.iter().min().unwrap();

    min.abs() + cum.iter().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![3, -5, 7, -4], 3),
            TestCase(5, vec![0, 0, 0, 0, 0], 0),
            TestCase(4, vec![-1, 1000000000, 1000000000, 1000000000], 3000000000),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
