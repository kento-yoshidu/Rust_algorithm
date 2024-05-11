// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dx

pub fn run(s: &str) -> Vec<(usize, usize)> {
    let mut stack = Vec::new();
    let mut ans = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        } else {
            ans.push((stack.last().unwrap()+1, i+1));
            stack.pop();
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, Vec<(usize, usize)>);

    #[test]
    fn test() {
        let tests = [
            TestCase("(())()", vec![(2, 3), (1, 4), (5, 6)]),
            TestCase("((()))", vec![(3, 4), (2, 5), (1, 6)]),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
