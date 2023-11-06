// https://atcoder.jp/contests/abc150/tasks/abc150_b

pub fn run(n: usize, s: String) -> usize {
    let mut ans = 0;

    let vec: Vec<char> = s.chars().collect();

    for i in 0..(n-2) {
        if String::from("ABC") == format!("{}{}{}", vec[i],vec[i+1],vec[i+2]) {
            ans += 1;
        }
    }

    ans
}

pub fn run2(_n: usize, s: String) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars
        .windows(3)
        .filter(|v| String::from("ABC") == format!("{}{}{}", v[0],v[1],v[2]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(10, String::from("ZABCDBABCQ")));
        assert_eq!(0, run(19, String::from("THREEONEFOURONEFIVE")));
        assert_eq!(5, run(33, String::from("ABCCABCBABCCABACBCBBABCBCBCBCABCB")));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(10, String::from("ZABCDBABCQ")));
        assert_eq!(0, run2(19, String::from("THREEONEFOURONEFIVE")));
        assert_eq!(5, run2(33, String::from("ABCCABCBABCCABACBCBBABCBCBCBCABCB")));
    }
}
