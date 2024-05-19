// https://atcoder.jp/contests/jsc2021/tasks/jsc2021_b

pub fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut ans = Vec::new();

    for num in 1..=1000 {
        if a.contains(&num) && !b.contains(&num) || !a.contains(&num) && b.contains(&num) {
            ans.push(num)
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, vec![1, 2], vec![1, 3], vec![2, 3]),
            TestCase(4, 4, vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![]),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
