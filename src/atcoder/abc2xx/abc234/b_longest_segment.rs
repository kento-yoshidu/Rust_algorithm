// https://atcoder.jp/contests/abc234/tasks/abc234_b

fn run(n: isize, a: Vec<Vec<isize>>) -> f64 {
    let mut max = 0_f64;

    for i in 0..n {
        for j in i+1..n {
            let tmp = ((a[i as usize][0] - a[j as usize][0]).pow(2)) + ((a[i as usize][1] - a[j as usize][1]).pow(2));

            max = max.max((tmp as f64).sqrt());
        }
    }

    (max * 10000000000.0).round() / 10000000000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, Vec<Vec<isize>>, f64);

    #[test]
    fn abc234_b() {
        let tests = [
            TestCase(3, vec![vec![0, 0], vec![0, 1], vec![1, 1]], 1.4142135624),
            TestCase(5, vec![vec![315, 271], vec![-2, -621], vec![-205, -511], vec![-952, 482], vec![165, 463]], 1455.7159750446),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
