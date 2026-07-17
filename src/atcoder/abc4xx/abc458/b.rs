// https://atcoder.jp/contests/abc458/tasks/abc458_b

fn run(h: usize, w: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![Vec::new(); h];

    for i in 0..h {
        for j in 0..w {
            let mut count = 4;

            if i == 0 {
                count -= 1;
            }

            if j == 0 {
                count -= 1;
            }

            if i+1 == h {
                count -= 1;
            }

            if j+1 == w {
                count -= 1;
            }

            ans[i].push(count);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>);

    #[test]
    fn abc458_b() {
        let tests = [
            TestCase(4, 5, vec![vec![2, 3, 3, 3, 2], vec![3, 4, 4, 4, 3], vec![3, 4, 4, 4, 3], vec![2, 3, 3, 3, 2]]),
            TestCase(1, 1, vec![vec![0]]),
            TestCase(12, 8, vec![ vec![2, 3, 3, 3, 3, 3, 3, 2], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![3, 4, 4, 4, 4, 4, 4, 3], vec![2, 3, 3, 3, 3, 3, 3, 2]]),
        ];

        for TestCase(h, w, expected) in tests {
            assert_eq!(run(h, w), expected);
        }
    }
}
