// https://atcoder.jp/contests/abc306/tasks/abc306_c

pub fn run(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut count = vec![0; n];
    let mut ans = Vec::new();

    for num in a {
        count[num-1] += 1;

        if count[num-1] == 2 {
            ans.push(num)
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 1, 3, 2, 3, 2, 2, 3, 1], vec![1, 3, 2]),
            TestCase(1, vec![1, 1, 1], vec![1]),
            TestCase(4, vec![2, 3, 4, 3, 4, 1, 3, 1, 1, 4, 2, 2], vec![3, 4, 1, 2]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
