// https://atcoder.jp/contests/arc004/tasks/arc004_1

pub fn run(n: usize, vec: Vec<Vec<i32>>) -> f64 {
    let mut ans: f64 = 0.0;

    for i in 0..n {
        for j in i+1..n {
            let num = (vec[j][0] - vec[i][0]).pow(2) + (vec[j][1] - vec[i][1]).pow(2);

            ans = ans.max((num as f64).sqrt()) as f64
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3.605551275463989, run(3, vec![vec![1, 1], vec![2, 4], vec![4, 3]]));
        assert_eq!(10.63014581273465, run(10, vec![vec![1, 8], vec![4, 0], vec![3, 7], vec![2, 4], vec![5, 9], vec![9, 1], vec![6, 2], vec![0, 2], vec![8, 6], vec![7, 8]]));
    }
}
