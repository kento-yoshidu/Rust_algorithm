// https://atcoder.jp/contests/abc003/tasks/abc003_3

pub fn run(n: usize, k: usize, r: Vec<usize>) -> f64 {
    let mut vec: Vec<f64> = r.iter().map(|num| *num as f64).collect();

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut score = 0.0;

    // 昇順の配列の後ろからk個を足す
    for i in vec.iter().skip(n-k) {
        score = (score + i) / 2.0;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1000.0, run(2, 2, vec![1000, 1500]));
        assert_eq!(750.0, run(2, 1, vec![1000, 1500]));
        assert_eq!(2820.03125, run(10, 5, vec![2604, 2281, 3204, 2264, 2200, 2650, 2229, 2461, 2439, 2211]));
    }
}
