// https://atcoder.jp/contests/abc133/tasks/abc133_b

fn run(n: usize, d: usize, x: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            let mut num = 0;

            for c in 0..d {
                num += ((x[i][c] - x[j][c]).abs()).pow(2);
            }

            for k in 1..1000 {
                if k*k == num {
                    ans += 1;
                }
            }
        }
    }

    ans
}

fn run2(n: usize, d: usize, x: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            let mut num = 0;

            for c in 0..d {
                num += ((x[i][c] - x[j][c]).abs()).pow(2);
            }

            let sq = (num as f64).sqrt().floor();

            if num != 0 && sq*sq == num.into() {
                ans += 1;
            }
        }
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<i32>>, i32);

    #[test]
    fn abc133_b() {
        let tests = [
            TestCase(3, 2, vec![vec![1, 2], vec![5, 5], vec![-2, 8]], 1),
            TestCase(3, 4, vec![vec![-3, 7, 8, 2], vec![-12, 1, 10, 2], vec![-2, 8, 9, 3]], 2),
            TestCase(5, 1, vec![vec![1], vec![2], vec![3], vec![4], vec![5]], 10),
            TestCase(2, 10, vec![vec![-20, 20, 20, 20, 20, 20, -15, -20, 20, -20], vec![20, -20, -20, -20, -20, -20, 20, 20, -20, 20]], 1),
        ];

        for TestCase(n, d, x, expected) in tests {
            assert_eq!(run(n, d, &x), expected);
            assert_eq!(run2(n, d, &x), expected);
        }
    }
}
