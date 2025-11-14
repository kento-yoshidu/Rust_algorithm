// https://atcoder.jp/contests/abc150/tasks/abc150_b

fn run(n: usize, s: &str) -> usize {
    let mut ans = 0;

    let vec: Vec<char> = s.chars().collect();

    for i in 0..(n-2) {
        if String::from("ABC") == format!("{}{}{}", vec[i],vec[i+1],vec[i+2]) {
            ans += 1;
        }
    }

    ans
}

fn run2(_n: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars
        .windows(3)
        .filter(|v| String::from("ABC") == format!("{}{}{}", v[0],v[1],v[2]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn abc150_b() {
        let tests = [
            TestCase(10, "ZABCDBABCQ", 2),
            TestCase(19, "THREEONEFOURONEFIVE", 0),
            TestCase(33, "ABCCABCBABCCABACBCBBABCBCBCBCABCB", 5),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
