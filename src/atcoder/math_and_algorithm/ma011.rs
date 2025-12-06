// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k

use num_integer::Roots;

fn is_prime(n: usize) -> bool {
    (2..=n.sqrt())
        .all(|x| n % x != 0)
}

fn run(n: usize) -> Vec<usize> {
    (2..=n)
        .filter(|x| is_prime(*x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn ma011() {
        let tests = [
            TestCase(10, vec![2, 3, 5 ,7]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
