// https://atcoder.jp/contests/abc293/tasks/abc293_a

fn run(s: &str) -> String {
    let mut ans = String::new();

    let str: Vec<char> = s.chars().map(|c| c).collect();

    for i in (0..str.len()).step_by(2) {
        ans.push(str[i+1]);
        ans.push(str[i]);
    }

    ans
}

fn run2(s: &str) -> String {
    let mut str: Vec<char> = s.chars().collect();

    for i in (0..str.len()).step_by(2) {
        let tmp = str[i];

        str[i] = str[i+1];
        str[i+1] = tmp;
    }

    str.iter().collect()
}

fn run3(s: &str) -> String {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| {
            vec![chunk[1], chunk[0]]
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("abcdef", "badcfe"),
            TestCase("aaaa", "aaaa"),
            TestCase("atcoderbeginnercontest", "taocedbrgeniencrnoetts"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
            assert_eq!(run3(s), expected);
        }
    }
}
