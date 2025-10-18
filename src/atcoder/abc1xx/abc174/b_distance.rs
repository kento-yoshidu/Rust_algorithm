// https://atcoder.jp/contests/abc174/tasks/abc174_b

fn run(_n: usize, k: usize, vec: &Vec<(isize, isize)>) -> usize {
    let mut ans = 0;

    for (x, y) in vec {
        let sqrt = ((x*x + y*y) as f64).sqrt();

        if k as f64 >= sqrt {
            ans += 1;
        }
    }

    ans
}

fn run2(_n: usize, d: usize, p: &Vec<(isize, isize)>) -> usize {
    p.into_iter()
        .filter(|(x, y)| {
            let sqrt = ((x*x + y*y) as f64).sqrt();

            d as f64 >= sqrt
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(isize, isize)>, usize);

    #[test]
    fn abc174_b() {
        let tests = [
            TestCase(4, 5, vec![(0, 5), (-2, 4), (3, 4), (4, -4)], 3),
            TestCase(12, 3, vec![(1, 1), (1, 1), (1, 1), (1, 1), (1, 2), (1, 3), (2, 1), (2, 2), (2, 3), (3, 1), (3, 2), (3, 3)], 7),
            TestCase(20, 100000, vec![(14309, -32939), (-56855, 100340), (151364, 25430), (103789, -113141), (147404, -136977), (-37006, -30929), (188810, -49557), (13419, 70401), (-88280, 165170), (-196399, 137941), (-176527, -61904), (46659, 115261), (-153551, 114185), (98784, -6820), (94111, -86268), (-30401, 61477), (-55056, 7872), (5901, -163796), (138819, -185986), (-69848, -96669)], 6),
        ];

        for TestCase(n, d, p, expected) in tests {
            assert_eq!(run(n, d, &p), expected);
            assert_eq!(run2(n, d, &p), expected);
        }
    }
}
