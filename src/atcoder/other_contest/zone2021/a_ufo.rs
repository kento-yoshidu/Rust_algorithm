// https://atcoder.jp/contests/zone2021/tasks/zone2021_a

fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.windows(4)
        .filter(|arr| {
            arr[0] == 'Z' && arr[1] == 'O' && arr[2] == 'N' && arr[3] == 'e'
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
            TestCase("abcdZONefghi", 1),
            TestCase("ZONeZONeZONe", 3),
            TestCase("helloAtZoner", 0),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
