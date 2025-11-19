// https://atcoder.jp/contests/abc158/tasks/abc158_d

use std::collections::VecDeque;

fn run(s: &str, _n: usize, query: Vec<(usize, Option<usize>, Option<char>)>) -> String {
    let mut ans = VecDeque::new();

    for c in s.chars() {
        ans.push_back(c);
    }

    let mut flag = true;

    for q in query.iter() {
        match q.0 {
            1 => {
                flag = !flag;
            },
            2 => {
                let p = q.1.unwrap();
                let c = q.2.unwrap();

                if p == 1 {
                    if flag {
                        ans.push_front(c);
                    } else {
                        ans.push_back(c);
                    }
                } else {
                    if flag {
                        ans.push_back(c);
                    } else {
                        ans.push_front(c);
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    if flag {
        ans.into_iter().collect()
    } else {
        ans.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, Vec<(usize, Option<usize>, Option<char>)>, &'static str);

    #[test]
    fn abc158_d() {
        let tests = [
            TestCase("a", 4, vec![(2, Some(1), Some('p')), (1, None, None), (2, Some(2), Some('c')), (1, None, None)], "cpa"),
            TestCase("a", 6, vec![(2, Some(2), Some('a')), (2, Some(1), Some('b')), (1, None, None), (2, Some(2), Some('c')), (1, None, None), (1, None, None)], "aabc"),
            TestCase("y", 1, vec![(2, Some(1),  Some('x'))], "xy"),
            TestCase("bptmmmfhvwugnwpfobepgxcfbhcfeasowgtpzjajsquuffpkvirvjbnhbvcrkbgqkghsqph", 1, vec![(2, Some(2), Some('g'))], "bptmmmfhvwugnwpfobepgxcfbhcfeasowgtpzjajsquuffpkvirvjbnhbvcrkbgqkghsqphg"),
        ];

        for TestCase(s, n, query, expected) in tests {
            assert_eq!(run(s, n, query), expected);
        }
    }
}
