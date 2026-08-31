// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/6/ITP1_6_C

fn run(_n: usize, bfrv: Vec<(usize, usize, usize, usize)>) -> Vec<Vec<Vec<usize>>> {
    let mut ans = vec![vec![vec![0; 10]; 3]; 4];

    for (h, f, r, v) in bfrv {
        ans[h-1][f-1][r-1] += v;
    }

    println!("{:?}", ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize, usize, usize)>, Vec<Vec<Vec<usize>>>);

    #[test]
    fn itp1_6_a() {
        let tests = [
            TestCase(3, vec![(1, 1, 3, 8), (3, 2, 2, 7), (4, 3, 8, 1)], vec![
            vec![vec![0, 0, 8, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]],
            vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]],
            vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 7, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]],
            vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0]]]),
        ];

        for TestCase(n, bfrv, expected) in tests {
            assert_eq!(run(n, bfrv), expected);
        }
    }
}
