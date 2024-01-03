// https://atcoder.jp/contests/abc143/tasks/abc143_c

pub fn run(_n: usize, s: &str) -> usize {
    let mut chars: Vec<char> = s.chars().collect();

    chars.dedup();

    chars.len()
}

fn run_lengths(s: Vec<char>) -> Vec<(char, usize)> {
    let mut run_lengths = vec![];
    let mut current = (s[0], 1);

    for i in 1..s.len() {
        if s[i] == current.0 {
            current.1 += 1;
        } else {
            run_lengths.push(current);
            current = (s[i], 1);
        }
    }

    run_lengths.push(current);

    run_lengths
}

pub fn run2(_n: usize, s: &str) -> usize {
    run_lengths(s.chars().collect()).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(10, "aabbbbaaca"));
        assert_eq!(1, run(5, "aaaaa"));
        assert_eq!(10, run(20, "xxzaffeeeeddfkkkkllq"));
    }

    #[test]
    fn test2() {
        assert_eq!(5, run2(10, "aabbbbaaca"));
        assert_eq!(1, run2(5, "aaaaa"));
        assert_eq!(10, run2(20, "xxzaffeeeeddfkkkkllq"));
    }
}
