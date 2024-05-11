// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ay

pub fn run<'a>(_q: usize, q_vec: Vec<(usize, Option<&'a str>)>) -> Vec<&'a str> {
    let mut stack = Vec::new();
    let mut ans = Vec::new();

    for (num, str) in q_vec.into_iter() {
        match num {
            1 => {
                stack.push(str.unwrap());
            },
            2 => {
                ans.push(*stack.last().unwrap())
            },
            3 => {
                stack.pop();
            },
            _ => unreachable!()
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Option<&'static str>)>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![(1, Some("futuremap")), (1, Some("howtospeak")), (2, None), (3, None), (2, None)], vec!["howtospeak", "futuremap"]),
        ];

        for TestCase(q, q_vec, expected) in tests {
            assert_eq!(run(q, q_vec), expected);
        }
    }
}
