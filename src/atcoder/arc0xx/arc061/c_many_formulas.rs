// https://atcoder.jp/contests/abc045/tasks/arc061_a

pub fn run(s: &str) -> usize {
    let len = s.len();
    let chars: Vec<char> = s.chars().collect();

    let mut ans = 0;

    for bit in 0..(1 << len-1) {
        let mut current = chars[0].to_string();
        let mut sum = 0;

        for i in 0..(len-1) {
            if bit & (1 << i) != 0 {
                current.push(chars[i+1]);
            } else {
                sum += current.parse::<usize>().unwrap();
                current = chars[i+1].to_string();
            }
        }

        sum += current.parse::<usize>().unwrap();
        ans+= sum;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("125", 176),
            TestCase("9999999999", 12656242944),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
