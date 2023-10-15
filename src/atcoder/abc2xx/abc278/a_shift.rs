// https://atcoder.jp/contests/abc278/tasks/abc278_a

use std::collections::VecDeque;

pub fn run(_n: usize, k: usize, a: Vec<usize>) -> String {
    let mut vec_deque = VecDeque::from(a);

    for _i in 0..k {
        vec_deque.pop_front();
        vec_deque.push_back(0);
    }

    vec_deque.iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("8 0 0"), run(3, 2, vec![2, 7, 8]));
        assert_eq!(String::from("0 0 0"), run(3, 4, vec![9, 9, 9]));
        assert_eq!(String::from("6 7 8 9 0 0 0 0 0"), run(9, 5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
}
