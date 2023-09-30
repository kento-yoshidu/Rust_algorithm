// https://atcoder.jp/contests/abc322/tasks/abc322_a

pub fn run(n: usize, s: String) -> isize {
    if !s.contains("ABC") {
        return -1;
    }

    let vec: Vec<char> = s.chars().collect();

    for (i, index) in (0..n-2).enumerate() {
        if vec[i] == 'A' && vec[i+1] == 'B' && vec[i+2] == 'C' {
            return (index + 1) as isize;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(8, String::from("ABABCABC")));
        assert_eq!(-1, run(8, String::from("ACB")));
        assert_eq!(13, run(20, String::from("BBAAABBACAACABCBABAB")));
    }
}
