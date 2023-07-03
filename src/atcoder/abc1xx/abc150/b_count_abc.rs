// https://atcoder.jp/contests/abc150/tasks/abc150_b

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(10, String::from("ZABCDBABCQ")));
        assert_eq!(0, run(19, String::from("THREEONEFOURONEFIVE")));
        assert_eq!(5, run(33, String::from("ABCCABCBABCCABACBCBBABCBCBCBCABCB")));
    }
}
