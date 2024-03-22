// https://atcoder.jp/contests/abc342/tasks/abc342_b

pub fn run(n: usize, p: Vec<usize>, _q: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut vec = vec![0; n];

    for (i, num) in p.iter().enumerate() {
        vec[num-1] = i;
    }

    ab.iter()
        .map(|(a, b)| {
            if vec[a-1] > vec[b-1] {
                *b
            } else {
                *a
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![2, 1, 3], 3, vec![(2, 3), (1, 2), (1, 3)], vec![2, 2, 1]),
            TestCase(7, vec![3, 7, 2, 1, 6, 5, 4], 13, vec![ (2, 3), (1, 2), (1, 3), (3, 6), (3, 7), (2, 4), (3, 7), (1, 3), (4, 7), (1, 6), (2, 4), (1, 3), (1, 3)], vec![3, 2, 3, 3, 3, 2, 3, 3, 7, 1, 2, 3, 3]),
        ];

        for TestCase(n, p, q, ab, expected) in tests {
            assert_eq!(run(n, p, q, ab), expected);
        }
    }
}
