// https://atcoder.jp/contests/abc180/tasks/abc180_b

fn run(_n: usize, x: Vec<isize>) -> (usize, f64, usize) {
    let abs_x: Vec<usize> = x.into_iter().map(|num| num.abs() as usize).collect();

    let ans_m: usize = abs_x.iter().sum();

    let ans_y = abs_x.iter()
        .map(|num| (num * num) as f64)
        .sum::<f64>().sqrt();

    let ans_c = abs_x.into_iter().max().unwrap();

    (ans_m, ans_y, ans_c)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, (usize, f64, usize));

    #[test]
    fn abc180_b() {
        let tests = [
            TestCase(2, vec![2, -1], (3, 2.23606797749979, 2)),
            TestCase(10, vec![3, -1, -4, 1, -5, 9, 2, -6, 5, -3], (39, 14.38749456993816, 9)),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
