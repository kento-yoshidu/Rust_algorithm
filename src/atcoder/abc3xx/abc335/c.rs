// https://atcoder.jp/contests/abc335/tasks/abc335_c

use std::collections::VecDeque;

fn run(n: usize, _q: usize, query: Vec<(&str, &str)>) -> Vec<(isize, isize)> {
    let mut pos = VecDeque::new();
    let mut ans = Vec::new();

    for i in 1..=n {
        pos.push_back((i as isize, 0));
    }

    for (t0, t1) in query {
        match t0 {
            "1" => {
                let mut p = pos[0];

                match t1 {
                    "U" => {
                        p.1 += 1;
                        pos.push_front(p);
                    },
                    "R" => {
                        p.0 += 1;
                        pos.push_front(p);
                    },
                    "D" => {
                        p.1 -= 1;
                        pos.push_front(p);
                    },
                    "L" => {
                        p.0 -= 1;
                        pos.push_front(p);
                    }
                    _ => unreachable!(),
                }

                pos.pop_back();
            },
            "2" => {
                ans.push(pos[t1.parse::<usize>().unwrap() - 1]);
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(&'static str, &'static str)>, Vec<(isize, isize)>);

    #[test]
    fn abc335_c() {
        let tests = [
            TestCase(5, 9, vec![("2", "3"), ("1", "U"), ("2", "3"), ("1", "R"), ("1", "D"), ("2", "3"), ("1", "L"), ("2", "1"), ("2", "5")], vec![(3, 0), (2, 0), (1, 1), (1, 0), (1, 0)]),
        ];

        for TestCase(n, q, vec, expected) in tests {
            assert_eq!(run(n, q, vec), expected);
        }
    }
}
