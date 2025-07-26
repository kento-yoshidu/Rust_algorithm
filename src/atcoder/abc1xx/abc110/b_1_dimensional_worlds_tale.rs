// https://atcoder.jp/contests/abc110/tasks/abc110_b

fn run(_n: isize, _m: isize, x: isize, y: isize, x_vec: Vec<isize>, y_vec: Vec<isize>) -> &'static str {
    let x_max = x_vec.iter().max().unwrap();
    let y_min = y_vec.iter().min().unwrap();

    if (x+1..=y)
        .any(|num| {
            x_max < &num && &num <= y_min
        }) {
            "No War"
        } else {
            "War"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, Vec<isize>, Vec<isize>, &'static str);

    #[test]
    fn abc110_b() {
        let tests = [
            TestCase(3, 2, 10, 20, vec![8, 15, 13], vec![16, 22], "No War"),
            TestCase(4, 2, -48, -1, vec![-20, -35, -91, -23], vec![-22, 66], "War"),
            TestCase(5, 3, 6, 8, vec![-10, 3, 1, 5, -100], vec![100, 6, 14], "War"),
        ];

        for TestCase(n, m, x, y, x_vec, y_vec, expected) in tests {
            assert_eq!(run(n, m, x, y, x_vec, y_vec), expected);
        }
    }
}
