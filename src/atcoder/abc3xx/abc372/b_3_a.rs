// https://atcoder.jp/contests/abc372/tasks/abc372_b

fn run(m: isize) -> (usize, Vec<isize>) {
    let mut rest = m;
    let mut vec = Vec::new();

    for i in (0..=10).rev() {
        let num = 3_isize.pow(i);

        while num <= rest {
            rest -= num;
            vec.push(i as isize);
        }
    }

    (vec.len(), vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, (usize, Vec<isize>));

    #[test]
    fn test() {
        let tests = [
            TestCase(6, (2, vec![1, 1])),
            TestCase(100, (4, vec![4, 2, 2, 0])),
            TestCase(59048, (20, vec![9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0])),
        ];

        for TestCase(m, expected) in tests {
            assert_eq!(run(m), expected);
        }
    }
}
