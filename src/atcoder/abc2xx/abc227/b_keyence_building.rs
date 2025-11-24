// https://atcoder.jp/contests/abc227/tasks/abc227_b

fn run(n: usize, vec: Vec<usize>) -> usize {
    let mut count = 0;

    for i in 0..vec.len() {
        'inner: for a in 1..vec[i] {
            for b in 1..vec[i] {
                if 4*a*b + 3*a + 3*b == vec[i] {
                    count += 1;
                    break 'inner;
                }
            }
        }
    }

    n - count
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc227_b() {
        let tests = [
            TestCase(3, vec![10, 20, 39], 1),
            TestCase(5, vec![666, 777, 888, 777, 666], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
