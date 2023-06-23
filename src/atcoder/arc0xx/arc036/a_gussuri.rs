// https://atcoder.jp/contests/arc036/tasks/arc036_a

use std::collections::VecDeque;

#[allow(dead_code)]
pub fn run(_n: usize, k: usize, vec: Vec<usize>) -> isize {
    let mut current = VecDeque::new();

    for (i, time) in vec.into_iter().enumerate() {
        current.push_back(time);

        if i <= 1 {
            continue
        }

        if current.iter().sum::<usize>() < k {
            return (i+1) as isize
        }

        current.pop_front();
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(5, 1080, vec![300, 420, 420, 180, 360]));
        assert_eq!(-1, run(5, 180, vec![60, 60, 60, 60, 60]));
        assert_eq!(3, run(5, 4230, vec![360, 360, 360, 360, 360]));
    }
}
