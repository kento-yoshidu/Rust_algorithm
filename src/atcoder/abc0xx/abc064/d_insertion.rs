// https://atcoder.jp/contests/abc064/tasks/abc064_d

use std::collections::VecDeque;

fn run(n: usize, s: &str) -> String {
    let vec: Vec<char> = s.chars().collect();

    let mut deque: VecDeque<char> = VecDeque::new();

    let mut l = 0;
    let mut r = 0;

    for i in 0..n {
        match vec[i] {
            '(' => {
                l += 1;
                deque.push_back('(');
            },
            ')' => {
                r += 1;
                deque.push_back(')');

                if r > l {
                    l += 1;
                    deque.push_front('(');
                }
            },
            _ => unreachable!(),
        }
    }

    let mut count = 0;

    for &c in deque.iter().rev() {
        if c == '(' {
            count += 1;
        } else {
            break;
        }
    }

    for _ in 0..count {
        deque.push_back(')');
        r += 1;
    }

    for _ in 0..l-r {
        deque.push_back(')');
    }

    deque.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, "())", "(())"),
            TestCase(6, ")))()))", "(((()))())"),
            TestCase(8, "))))((((", "(((())))(((())))"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
