#[allow(dead_code)]
pub fn run(_n: i32, _m: i32, c: i32, b: Vec<i32>, vec: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for v in vec.iter() {
        let mut total = 0;

        for (i, num) in v.iter().enumerate() {
            total += num * b[i];
        }

        if (total as i32 + c) > 0 {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(2, 3, -10, vec![1, 2, 3], vec![vec![3, 2, 1], vec![1, 2, 2]]));
        assert_eq!(2, run(5, 2, -4, vec![-2, 5], vec![vec![100, 41], vec![100, 40], vec![-3, 0], vec![-6, -2], vec![18, -13]]));
        assert_eq!(0, run(3, 3, 0, vec![100, -100, 0], vec![vec![0, 100, 100], vec![100, 100, 100], vec![-100, 100, 100]]))
    }
}
