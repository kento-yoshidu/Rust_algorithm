// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_r

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut map = HashMap::new();

    for num in a {
        *map.entry(num).or_insert(0) += 1;
    }

    let get = |x| *map.get(&x).unwrap_or(&0);

    get(100) * get(400) + get(200) * get(300)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn ma018() {
        let tests = [
            TestCase(5, vec![100, 300, 400, 400, 200], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
