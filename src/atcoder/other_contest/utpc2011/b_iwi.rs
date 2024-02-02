// https://atcoder.jp/contests/utpc2011/tasks/utpc2011_2

pub fn run(s: &str) -> usize {
    let mut ans = 0;

    let len = s.len();

    for i in 0..len/2 {
        let pair = (s.chars().nth(i).unwrap(), s.chars().nth(len-i-1).unwrap());

        match pair {
            ('i', 'i') | ('w' , 'w') | ('(', ')') | (')', '(') => (),
            _ => ans += 1,
        }
    }

    if len % 2 == 1 {
        let s = s.chars().nth(len/2).unwrap();

        if s != 'i' && s != 'w' {
            ans += 1;
        }
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
            TestCase("(iwi)", 0),
            TestCase("ii(((((ww", 5),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
