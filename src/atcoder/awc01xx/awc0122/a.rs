// https://atcoder.jp/contests/awc0122/tasks/awc0122_a

fn run(n: usize, _m: usize, ab: Option<Vec<(usize, usize)>>) -> usize {
    let Some(ab) = ab else {
        return 1;
    };

    let mut arr = vec![false; n];

    arr[0] = true;

    for (a, b) in ab {
        if arr[a] {
            arr[b] = true;
        }
    }

    arr.into_iter().filter(|b| *b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, usize)>>, usize);

    #[test]
    fn awc0122_a() {
        let tests = [
            TestCase(4, 3, Some(vec![(0, 1), (1, 2), (3, 2)]), 3),
            TestCase(4, 3, Some(vec![(1, 2), (0, 1), (2, 3)]), 2),
            TestCase(8, 13, Some(vec![(0, 3), (2, 4), (3, 2), (4, 5), (5, 1), (1, 7), (6, 7), (7, 6), (0, 2), (2, 4), (4, 5), (5, 6), (6, 7)]), 7),
            TestCase(15, 28, Some(vec![(0, 4), (1, 2), (4, 1), (2, 3), (3, 5), (6, 7), (5, 6), (8, 9), (7, 8), (10, 11), (11, 12), (12, 13), (13, 14), (0, 10), (9, 10), (14, 2), (4, 6), (6, 1), (1, 9), (9, 11), (11, 13), (13, 12), (12, 14), (14, 5), (5, 6), (6, 7), (7, 8), (8, 9)]), 13),
            TestCase(1, 0, None, 1),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
