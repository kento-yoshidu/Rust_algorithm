// https://atcoder.jp/contests/abc138/tasks/abc138_c

pub fn run(_n: usize, v: Vec<usize>) -> f64 {
    let mut vec: Vec<f64> = v.iter().map(|num| *num as f64).collect();

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    vec.iter()
        .skip(1)
        .fold(vec[0], |state, num| {
            (state + num) / 2.0
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3.5, run(2, vec![3, 4]));
        assert_eq!(375.0, run(3, vec![500, 300, 200]));
        assert_eq!(138.0, run(5, vec![138, 138, 138, 138, 138]));
    }
}
