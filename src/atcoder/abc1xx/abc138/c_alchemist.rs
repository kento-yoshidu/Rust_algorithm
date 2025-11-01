// https://atcoder.jp/contests/abc138/tasks/abc138_c

fn run(_n: usize, v: Vec<usize>) -> f64 {
    let mut vec: Vec<f64> = v.into_iter().map(|num| num as f64).collect();

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

    struct TestCase(usize, Vec<usize>, f64);

    #[test]
    fn abc138_c() {
        let tests = [
            TestCase(2, vec![3, 4], 3.5),
            TestCase(3, vec![500, 300, 200], 375.0),
            TestCase(5, vec![138, 138, 138, 138, 138], 138.0),
        ];

        for TestCase(n, v, expected) in tests {
            assert_eq!(run(n, v), expected);
        }
    }
}
