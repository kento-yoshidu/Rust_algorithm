// https://atcoder.jp/contests/arc007/tasks/arc007_1

pub fn run(x: char, s: &'static str) -> String {
    s.chars()
        .filter(|c| { *c != x })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase('a', "abcdefgabcdefg", "bcdefgbcdefg"),
            TestCase('g', "aassddffgg", "aassddff"),
            TestCase('a', "aaaaa", ""),
            TestCase('l', "qwertyuiopasdfghjklzxcvbnm", "qwertyuiopasdfghjkzxcvbnm"),
            TestCase('d', "qwsdtgcszddddsdfgvbbnj", "qwstgcszsfgvbbnj"),
        ];

        for TestCase(x, s, expected) in tests {
            assert_eq!(run(x, s), expected);
        }
    }
}
