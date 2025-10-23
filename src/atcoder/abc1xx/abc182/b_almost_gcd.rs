// https://atcoder.jp/contests/abc182/tasks/abc182_b

fn run(_n: usize, a: Vec<usize>) -> usize {
    let c = a.clone();

    let max = c.iter().max().unwrap();

    let mut tmp = 0;
    let mut ans = 0;

    for i in 2..=*max {
        let count = a.iter().filter(|&&num| num % i == 0).count();

        if count > tmp {
            tmp = count;
            ans = i;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![3, 12, 7], 3),
            TestCase(5, vec![8, 9, 18, 90, 72], 2),
            TestCase(5, vec![1000, 1000, 1000, 1000, 1000], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
