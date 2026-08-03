// https://atcoder.jp/contests/awc0089/tasks/awc0089_a

fn run(_n: usize, _q: usize, a: Vec<isize>, d: Vec<usize>) -> Vec<isize> {
    let sum: isize = a.iter().sum();

    d.into_iter()
        .scan(sum,  |state, d| {
            *state -= a[d-1];
            Some(*state)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<isize>, Vec<usize>, Vec<isize>);

    #[test]
    fn awc0089_a() {
        let tests = [
            TestCase(5, 3, vec![100, -50, 200, -30, 80], vec![2, 4, 1], vec![350, 380, 280]),
            TestCase(7, 4, vec![1000, -500, 300, -200, 400, -100, 200], vec![3, 1, 5, 7], vec![800, -200, -600, -800]),
            TestCase(10, 5, vec![1000000000, -999999999, 500000000, -400000000, 300000000, -200000000, 100000000, -50000000, 25000000, -12500000], vec![1, 3, 5, 7, 9], vec![-737499999, -1237499999, -1537499999, -1637499999, -1662499999]),
        ];

        for TestCase(n, q, a, d, expected) in tests {
            assert_eq!(run(n, q, a, d), expected);
        }
    }
}
