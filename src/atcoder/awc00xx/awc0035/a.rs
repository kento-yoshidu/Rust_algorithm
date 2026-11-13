// https://atcoder.jp/contests/awc0035/tasks/awc0035_a

fn run(_n: usize, _m: usize, p: Vec<usize>, uvw: Vec<(usize, usize, usize)>) -> usize {
    let mut p = p.clone();

    for (u, v, w) in uvw {
        p[u-1] -= w;
        p[v-1] += w;
    }

    let max = *p.iter().max().unwrap();

    p.into_iter()
        .position(|x| x == max)
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn awc0035_a() {
        let tests = [
            TestCase(3, 2, vec![10, 20, 30], vec![(1, 2, 5), (3, 2, 10)], 2),
            TestCase(3, 2, vec![10, 20, 10], vec![(2, 1, 5), (2, 3, 5)], 1),
            TestCase(6, 5, vec![100, 200, 150, 300, 250, 50], vec![(4, 2, 50), (1, 3, 30), (5, 6, 100), (2, 1, 80), (3, 5, 40)], 4),
            TestCase(10, 8, vec![500, 300, 800, 200, 600, 100, 450, 350, 700, 250], vec![(3, 1, 200), (5, 7, 150), (9, 2, 300), (4, 6, 100), (1, 10, 400), (8, 3, 50), (7, 9, 100), (6, 5, 50)], 3),
            TestCase(1, 0, vec![1000000000], vec![], 1),
        ];

        for TestCase(n, m, c, t, expected) in tests {
            assert_eq!(run(n, m, c, t), expected);
        }
    }
}
