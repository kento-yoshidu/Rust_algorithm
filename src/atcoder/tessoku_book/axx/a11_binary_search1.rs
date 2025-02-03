// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_k

fn run(n: usize, x: usize, a: &Vec<usize>) -> usize {
    let mut left = 1;
    let mut right = n;

    loop {
        let middle = (left + right) / 2;

        if x == a[middle] {
            return middle + 1;
        }

        if x < a[middle] {
            right = middle - 1
        } else {
            left = middle + 1
        }
    }
}

fn run2(_n: usize, x: usize, a: &Vec<usize>) -> usize {
    a.binary_search(&x).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(15, 47, vec![11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67], 11),
            TestCase(10, 80, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 8),
        ];

        for TestCase(n, x, a, expected) in tests {
            // assert_eq!(run(n, x, &a), expected);
            assert_eq!(run2(n, x, &a), expected);
        }
    }
}
