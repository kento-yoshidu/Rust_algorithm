#[allow(dead_code)]
pub fn run(_n: usize, m: usize, vec: Vec<Vec<usize>>) -> usize {
    let mut ans = 0;

    for i in 0..(m-1) {
        for j in (i+1)..m {
            // 2つの列を選択し、大きいものを選択し加える
            let mut score = 0;

            for v in vec.iter() {
                // 同じ行の中で大きい方を加える
                score += v[i].max(v[j]);
            }

            ans = ans.max(score);
        }
    };

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(84, run(1, 2, vec![vec![80, 84]]));
        assert_eq!(250, run(3, 4, vec![vec![37, 29, 70, 41], vec![85, 69, 76, 50], vec![53, 10, 95, 100]]));
        assert_eq!(581000000, run(8, 2, vec![vec![31000000, 41000000], vec![59000000, 26000000], vec![53000000, 58000000], vec![97000000, 93000000], vec![23000000, 84000000], vec![62000000, 64000000], vec![33000000, 83000000], vec![27000000, 95000000]]));
    }
}
