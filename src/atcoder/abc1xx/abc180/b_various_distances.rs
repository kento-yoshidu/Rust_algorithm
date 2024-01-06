// https://atcoder.jp/contests/abc180/tasks/abc180_b

pub fn run(_n: usize, x: Vec<isize>) -> (usize, f64, usize) {
    let abs_x: Vec<usize> = x.iter().map(|num| num.abs() as usize).collect();

    let ans_m: usize = abs_x.iter().sum();

    let ans_y = abs_x.iter()
        .map(|num| (num * num) as f64)
        .sum::<f64>().sqrt();

    let ans_c = abs_x.iter().max().unwrap();

    (ans_m, ans_y, *ans_c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((3, 2.23606797749979, 2), run(2, vec![2, -1]));
        assert_eq!((39, 14.38749456993816, 9), run(10, vec![3, -1, -4, 1, -5, 9, 2, -6, 5, -3]));
    }
}
