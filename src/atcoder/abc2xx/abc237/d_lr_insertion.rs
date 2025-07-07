// https://atcoder.jp/contests/abc237/tasks/abc237_d

use std::collections::VecDeque;

fn run(n: usize, s: &str) -> String {
    let mut dequeue = VecDeque::new();
    dequeue.push_front(n);

    let s: Vec<(usize, char)> = s.chars().enumerate().collect();

    for (i, c) in s.iter().rev() {
        match *c {
            'L' => dequeue.push_back(*i),
            'R' => dequeue.push_front(*i),
            _ => unreachable!(),
        }
    }

    dequeue.into_iter().map(|n| char::from_digit(n as u32, 10).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "LRRLR", "124530"),
            TestCase(7, "LLLLLLL", "76543210"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
