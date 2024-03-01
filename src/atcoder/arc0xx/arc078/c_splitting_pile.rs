// https://atcoder.jp/contests/abc067/tasks/arc078_a

fn run(_n: usize, a: Vec<isize>) -> isize {
    let mut ans = std::isize::MAX;

    let mut l = 0;
    let mut r = a.iter().sum::<isize>();

    for num in a.iter() {
        l += num;
        r -= num;

        ans = ans.min((l-r).abs());
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![1, 2, 3, 4, 5, 6], 1),
            TestCase(2, vec![20, -20], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
