// https://atcoder.jp/contests/abc348/tasks/abc348_b

pub fn run(n: usize, xy: Vec<(isize, isize)>) -> Vec<usize> {
    let mut ans = Vec::new();

    for i in 0..n {
        let mut dis = 0.0;
        let mut pos = 0;

        for j in 0..n {
            let d = ((xy[i].0 - xy[j].0).pow(2) as f64 + (xy[i].1 - xy[j].1).pow(2) as f64).sqrt();

            if dis < d {
                dis = d;
                pos = j;
            }
        }

        ans.push(pos + 1)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(0, 0), (2, 4), (5, 0), (3, 4)], vec![3, 3, 1, 1]),
            TestCase(6, vec![(3, 2), (1, 6), (4, 5), (1, 3), (5, 5), (9, 8)], vec![6, 6, 6, 6, 6, 4]),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
