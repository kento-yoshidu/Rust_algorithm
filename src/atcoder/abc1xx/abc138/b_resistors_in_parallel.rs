// https://atcoder.jp/contests/abc138/tasks/abc138_b

fn run(_n: usize, a: Vec<usize>) -> f64 {
    let vec: Vec<f64> = a.into_iter().map(|num| num as f64).collect();

    let total: f64 = vec.into_iter()
        .map(|num| {
            1.0 / num
        })
        .sum();

    1.0 / total
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, f64);

    #[test]
    fn abc138_b() {
        let tests = [
            TestCase(2, vec![10, 30], 7.5),
            TestCase(3, vec![200, 200, 200], 66.66666666666667),
            TestCase(1, vec![1000], 1000.0,),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
