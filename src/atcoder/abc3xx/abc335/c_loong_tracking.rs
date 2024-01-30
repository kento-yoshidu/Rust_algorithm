// https://atcoder.jp/contests/abc335/tasks/abc335_c

use std::collections::VecDeque;

pub fn run(n: usize, _q: usize, vec: Vec<(&str, &str)>) -> Vec<(isize, isize)> {
    let mut pos = VecDeque::new();
    let mut ans = Vec::new();

    for i in 1..=n {
        pos.push_back((i as isize, 0));
    }

    for (t0, t1) in vec {
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

    #[test]
    fn test() {
        assert_eq!(vec![(3, 0), (2, 0), (1, 1), (1, 0), (1, 0)], run(5, 9, vec![("2", "3"), ("1", "U"), ("2", "3"), ("1", "R"), ("1", "D"), ("2", "3"), ("1", "L"), ("2", "1"), ("2", "5")]));
    }
}
