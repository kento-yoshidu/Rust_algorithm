// https://atcoder.jp/contests/arc046/tasks/arc046_a

// Refactoring

use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(n: usize) -> usize {
    let mut count = 0;

    for i in 1.. {
        let len = i.to_string().chars().collect::<HashSet<char>>().len();

        if len == 1 {
            count += 1;
        }

        if count == n {
            return i
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(1));
        assert_eq!(22, run(11));
        assert_eq!(555555, run(50));
    }
}
