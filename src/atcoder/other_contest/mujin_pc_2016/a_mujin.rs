// https://atcoder.jp/contests/mujin-pc-2016/tasks/mujin_pc_2016_a

fn run(c: char) -> &'static str {
    if c == 'O' || c == 'P' || c == 'K' || c == 'L' {
        "Right"
    } else {
        "Left"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase('A', "Left"),
            TestCase('P', "Right"),
            TestCase('H', "Left"),
        ];

        for TestCase(c, expected) in tests {
            assert_eq!(run(c), expected);
        }
    }
}
