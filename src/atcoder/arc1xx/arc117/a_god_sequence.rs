// https://atcoder.jp/contests/arc117/tasks/arc117_a

pub fn run(a: isize, b: isize) -> Vec<isize> {
    let mut ans: Vec<isize> = Vec::new();

    if a > b {
        for i in 0..a {
            ans.push(i+1);
        }
        for i in 0..b-1 {
            ans.push(-(i+1));
        }
        ans.push((b..=a)
            .map(|i| -i)
            .sum());
    } else {
        for i in 0..a-1 {
            ans.push(i+1);
        }
        for i in 0..b {
            ans.push(-(i+1));
        }
        ans.push((a..=b)
            .map(|i| i)
            .sum());
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 1, vec![-1, 1]),
            TestCase(1, 4, vec![-1, -2, -3, -4, 10]),
            TestCase(7, 5, vec![1, 2, 3, 4, 5, 6, 7, -1, -2, -3, -4, -18]),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
