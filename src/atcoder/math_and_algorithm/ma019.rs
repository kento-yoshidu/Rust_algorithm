// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_s

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut count = [0; 4];

    for num in a {
        count[num] += 1;
    }

    let a = count[1] * (count[1] - 1) / 2;
    let b = count[2] * (count[2] - 1) / 2;
    let c = count[3] * (count[3] - 1) / 2;

    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn ma019() {
        let tests = [
            TestCase(6, vec![1, 3, 2, 1, 1, 2], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
