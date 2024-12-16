// https://atcoder.jp/contests/abc373/tasks/abc373_b

fn run(s: &str) -> usize {
    let alphabet: Vec<char> = ('A'..='Z').collect();
    let chars: Vec<char> = s.chars().collect();

    (0..25)
        .map(|i| {
            let pos1 = chars.iter().position(|c| *c == alphabet[i]).unwrap();
            let pos2 = chars.iter().position(|c| *c == alphabet[i+1]).unwrap();

            (pos1 as isize - pos2 as isize).abs() as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 25),
            TestCase("MGJYIZDKSBHPVENFLQURTCWOAX", 223),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
