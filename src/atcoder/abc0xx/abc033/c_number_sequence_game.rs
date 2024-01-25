// https://atcoder.jp/contests/abc033/tasks/abc033_c

pub fn run(s: &str) -> usize {
    let vec: Vec<&str> = s.split('+').collect();

    vec.iter()
        .filter(|str| {
            !str.contains('0')
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("0+0+2*0", 0),
            TestCase("3*1+1*2", 2),
            TestCase("3*1*4+0+2*0+5*2+9*8*6+1+3", 5),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(expected, run(s));
        }
    }
}
