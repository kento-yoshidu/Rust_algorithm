// https://atcoder.jp/contests/abc166/tasks/abc166_c

use std::collections::HashSet;

fn run(n: usize, _m: usize, h: Vec<usize>, ab: Vec<(usize, usize)>) -> usize {
    let mut vec = vec![HashSet::new(); n];

    for (a, b) in ab {
        vec[a-1].insert(b-1);
        vec[b-1].insert(a-1);
    }

    let mut count = 0;

    for i in 0..n {
        let max = vec[i].iter().map(|j| h[*j]).max().unwrap_or(0);

        if h[i] > max {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(usize, usize)>, usize);

    #[test]
    fn abc166_c() {
        let tests = [
            TestCase(4, 3, vec![1, 2, 3, 4], vec![(1, 3), (2, 3), (2, 4)], 2),
            TestCase(6, 5, vec![8, 6, 9, 1, 2, 1], vec![(1, 3), (4, 2), (4, 3), (4, 6), (4, 6)], 3),
            TestCase(2, 1, vec![5, 5], vec![(1, 2)], 0),
            TestCase(2, 1, vec![5, 6], vec![(1, 2)], 1),
            TestCase(9, 7, vec![926701371, 164012478, 336181839, 971654588, 132991922, 457718299, 585010933, 98598159, 74844601], vec![(2, 4), (9, 3), (9, 2), (3, 4), (9, 5), (6, 3), (7, 2)], 6),
            TestCase(7, 8, vec![401867797, 148956136, 332807122, 441865860, 234019466, 915569177, 43788856], vec![(1, 5), (5, 6), (4, 7), (1, 2), (1, 5), (7, 6), (7, 2), (3, 4)], 3),
            TestCase(10, 9, vec![877034223, 133899794, 476916052, 912077132, 335047010, 225936408, 355083132, 818106283, 721973481, 718651389], vec![(7, 9), (7, 4), (5, 6), (5, 9), (2, 5), (6, 1), (1, 9), (2, 8), (5, 6)], 5),
        ];

        for TestCase(n, m, h, ab, expected) in tests {
            assert_eq!(run(n, m, h, ab), expected);
        }
    }
}
