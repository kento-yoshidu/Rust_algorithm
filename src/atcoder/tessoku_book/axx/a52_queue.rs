// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_az

use std::collections::VecDeque;

pub fn run<'a>(_q: usize, q_vec: Vec<(usize, Option<&'a str>)>) -> Vec<&'a str> {
    let mut ans = Vec::new();
    let mut queue = VecDeque::new();

    for (num, str) in q_vec {
        match num {
            1 => queue.push_back(str.unwrap()),
            2 => ans.push(queue[0]),
            3 => {
                queue.pop_front();
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Option<&'static str>)>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![(1, Some("taro")), (1, Some("hanako")), (2, None), (3, None), (2, None)], vec!["taro", "hanako"]),
        ];

        for TestCase(q, q_vec, expected) in tests {
            assert_eq!(run(q, q_vec), expected);
        }
    }
}
