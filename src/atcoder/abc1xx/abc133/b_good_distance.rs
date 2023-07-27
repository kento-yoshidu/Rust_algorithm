// https://atcoder.jp/contests/abc133/tasks/abc133_b

#[allow(unused, dead_code)]
pub fn run(n: i32, d: i32, vec: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            let mut num = 0;

            for c in 0..d {
                num += ((vec[i as usize][c as usize] - vec[j as usize][c as usize]).abs()).pow(2);
            }

            // 平方数かどうかの判定
            for k in 1..1000 {
                if k*k == num {
                    ans += 1;
                }
            }
        }
    }

    ans
}

#[allow(unused, dead_code)]
pub fn run2(n: i32, d: i32, vec: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            let mut num = 0;

            for c in 0..d {
                num += ((vec[i as usize][c as usize] - vec[j as usize][c as usize]).abs()).pow(2);
            }

            let sq = (num as f64).sqrt().floor();

            // 平方数かどうかの判定
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

    #[test]
    fn test() {
        assert_eq!(1, run(3, 2, vec![vec![1, 2], vec![5, 5], vec![-2, 8]]));
        assert_eq!(2, run(3, 4, vec![vec![-3, 7, 8, 2], vec![-12, 1, 10, 2], vec![-2, 8, 9, 3]]));
        assert_eq!(10, run(5, 1, vec![vec![1], vec![2], vec![3], vec![4], vec![5]]));
        assert_eq!(1, run(2, 10, vec![vec![-20, 20, 20, 20, 20, 20, -15, -20, 20, -20], vec![20, -20, -20, -20, -20, -20, 20, 20, -20, 20]]))
    }

    #[test]
    fn test2() {
        assert_eq!(1, run2(3, 2, vec![vec![1, 2], vec![5, 5], vec![-2, 8]]));
        assert_eq!(2, run2(3, 4, vec![vec![-3, 7, 8, 2], vec![-12, 1, 10, 2], vec![-2, 8, 9, 3]]));
        assert_eq!(10, run2(5, 1, vec![vec![1], vec![2], vec![3], vec![4], vec![5]]));
        assert_eq!(1, run2(2, 10, vec![vec![-20, 20, 20, 20, 20, 20, -15, -20, 20, -20], vec![20, -20, -20, -20, -20, -20, 20, 20, -20, 20]]))
    }
}
