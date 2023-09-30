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

pub fn run2(_n: usize, s: String) -> isize {
    (s.find("ABC").unwrap_or(!0-1) + 1) as isize
}

pub fn run3(_n: usize, s: String) -> isize {
    s.chars().collect::<Vec<char>>().windows(3).position(|v| {
        v == ['A', 'B', 'C']
    }).unwrap_or(!0-1) as isize + 1
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

    #[test]
    fn test2() {
        assert_eq!(3, run2(8, String::from("ABABCABC")));
        assert_eq!(-1, run2(8, String::from("ACB")));
        assert_eq!(13, run2(20, String::from("BBAAABBACAACABCBABAB")));
    }

    #[test]
    fn test3() {
        assert_eq!(3, run3(8, String::from("ABABCABC")));
        assert_eq!(-1, run3(8, String::from("ACB")));
        assert_eq!(13, run3(20, String::from("BBAAABBACAACABCBABAB")));
    }
}
