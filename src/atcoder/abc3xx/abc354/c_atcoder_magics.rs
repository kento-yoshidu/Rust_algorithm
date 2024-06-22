// https://atcoder.jp/contests/abc354/tasks/abc354_c

pub fn run(_n: usize, ac: Vec<(usize, usize)>) -> (usize, Vec<usize>) {
    let mut ans = Vec::new();

    let mut vec: Vec<(usize, (usize, usize))> = ac.into_iter()
        .enumerate()
        .collect();

    vec.sort_by(|a, b| a.1.1.cmp(&b.1.1));

    let mut v = 0;

    for (i, (a, _)) in vec.into_iter() {
        if v < a {
            ans.push(i+1);
            v = a;
        }
    }

    ans.sort();

    (ans.len(), ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, (usize, Vec<usize>));

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(2, 4), (1, 1), (3, 2)], (2, vec![2, 3])),
            TestCase(5, vec![(1, 1), (10, 2), (100, 3), (1000, 4), (10000, 5)], (5, vec![1, 2, 3, 4, 5])),
            TestCase(6, vec![(32, 101), (65, 78), (2, 29), (46, 55), (103, 130), (52, 40)], (4, vec![2, 3, 5, 6])),
        ];

        for TestCase(n, ac, expected) in tests {
            assert_eq!(run(n, ac), expected);
        }
    }
}
