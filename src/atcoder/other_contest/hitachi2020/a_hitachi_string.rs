// https://atcoder.jp/contests/hitachi2020/tasks/hitachi2020_a

fn run(s: &str) -> &'static str {
    let vec: Vec<String> = s.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect())
        .collect();

    if vec.into_iter().all(|str| str == "hi") {
        "Yes"
    } else {
        "No"
    }
}

fn run2(s: &str) -> &'static str {
    let chars: Vec<char> = s.chars().collect();

    if (0..s.len()).all(|i| {
        if i % 2 == 0 {
            chars[i] == 'h'
        } else {
            chars[i] == 'i'
        }
    }) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("hihi", "Yes"),
            TestCase("hi", "Yes"),
            TestCase("ha", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
