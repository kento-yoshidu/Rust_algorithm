// https://atcoder.jp/contests/abc093/tasks/abc093_b

fn run(a: usize, b: usize, k: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();

    for i in a ..=b {
        if i < a + k || b - k < i {
            vec.push(i.try_into().unwrap())
        }
    }

    vec
}

fn run2(a: usize, b: usize, k: usize) -> Vec<usize> {
    (a..=b)
        .filter(|num| {
            *num < a + k || b - k < *num
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase(usize, usize, usize, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 8, 2, vec![3, 4, 7, 8]),
            TestCase(4, 8, 3, vec![4, 5, 6, 7, 8]),
            TestCase(2, 9, 100,vec![2, 3, 4, 5, 6, 7, 8, 9]),
        ];

        for TestCase(a, b, k, expected) in tests {
            assert_eq!(run(a, b, k), expected);
            assert_eq!(run2(a, b, k), expected);
        }
    }
}
