#[allow(dead_code)]
pub fn run(n: i32, vec: Vec<Vec<i32>>) -> f64 {
    let mut max = 0_f64;

    for i in 0..n {
        for j in i+1..n {
            let tmp = ((vec[i as usize][0] - vec[j as usize][0]).pow(2)) + ((vec[i as usize][1] - vec[j as usize][1]).pow(2));

            max = max.max((tmp as f64).sqrt());
        }
    }

    (max * 10000000000.0).round() / 10000000000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1.4142135624, run(3, vec![vec![0, 0], vec![0, 1], vec![1, 1]]));
        assert_eq!(1455.7159750446, run(5, vec![vec![315, 271], vec![-2, -621], vec![-205, -511], vec![-952, 482], vec![165, 463]]));
    }
}
