// https://atcoder.jp/contests/arc004/tasks/arc004_1

pub fn run(n: usize, vec: Vec<(i32, i32)>) -> f64 {
    let mut ans: f64 = 0.0;

    for i in 0..n {
        for j in i+1..n {
            let num = (vec[j].0 - vec[i].0).pow(2) + (vec[j].1 - vec[i].1).pow(2);

            ans = ans.max((num as f64).sqrt()) as f64
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(i32, i32)>, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![(1, 1), (2, 4), (4, 3)], 3.605551275463989),
            TestCase(10, vec![(1, 8), (4, 0), (3, 7), (2, 4), (5, 9), (9, 1), (6, 2), (0, 2), (8, 6), (7, 8)], 10.63014581273465),
        ];

        for TestCase(n, vec, expected) in tests {
            assert_eq!(run(n, vec), expected);
        }
    }
}
