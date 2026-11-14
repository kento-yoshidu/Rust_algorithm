// https://atcoder.jp/contests/awc0037/tasks/awc0037_a

fn run(_n: usize, a: Vec<usize>) -> usize {
    let max = a.iter().max().unwrap();

    let count = a.iter().filter(|h| *h == max).count();

    if count == 1 {
        *max
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn awc0037_a() {
        let tests = [
            TestCase(5, vec![3, 1, 5, 2, 4], 5),
            TestCase(4, vec![3, 5, 5, 2], 0),
            TestCase(10, vec![14, 7, 23, 9, 42, 18, 42, 5, 31, 12], 0),
            TestCase(20, vec![100, 250, 300, 450, 123, 678, 999, 234, 567, 890, 111, 222, 333, 444, 555, 666, 777, 888, 1000000000, 999999999], 1000000000),
            TestCase(1, vec![1], 1),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
