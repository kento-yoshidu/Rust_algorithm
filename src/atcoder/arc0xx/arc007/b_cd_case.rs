// https://atcoder.jp/contests/arc007/tasks/arc007_2

fn run(n: usize, m: usize, d: Vec<usize>) -> Vec<usize> {
    let mut cur = 0;
    let mut vec: Vec<usize> = (1..=n).collect();

    if m == 0 {
        return vec;
    }

    for num in d {
        if let Some(pos) = vec.iter().position(|n| *n == num) {
            vec[pos] = cur;
            cur = num;
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 6, vec![2, 3, 5, 0, 1, 3], vec![0, 5, 2, 4, 1]),
            TestCase(3, 5, vec![0, 1, 1, 1, 2], vec![0, 1, 3]),
            TestCase(5, 0, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            TestCase(10, 7, vec![2, 8, 5, 3, 3, 8, 1], vec![8, 0, 5, 4, 3, 6, 7, 2, 9, 10]),
        ];

        for TestCase(n, m, d, expected) in tests {
            assert_eq!(run(n, m, d), expected);
        }
    }
}
