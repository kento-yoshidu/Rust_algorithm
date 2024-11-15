// https://atcoder.jp/contests/abc370/tasks/abc370_c

fn run(s: &str, t: &str) -> (usize, Option<Vec<String>>) {
    let mut a: Vec<char> = s.chars().collect();
    let b: Vec<char> = t.chars().collect();

    let n = a.len();

    let mut ans = Vec::new();


    for i in 0..a.len() {
        if a[i] > b[i] {
            a[i] = b[i];
            ans.push(a.iter().collect::<String>());
        }
    }

    for i in 0..a.len() {
        if a[n-i-1] != b[n-i-1] {
            a[n-i-1] = b[n-i-1];
            ans.push(a.iter().collect::<String>());
        }
    }

    if ans.len() == 0 {
        (0, None)
    } else {
        (ans.len(), Some(ans))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, (usize, Option<Vec<String>>));

    #[test]
    fn test() {
        let tests = [
            TestCase("adbe", "bcbc", (3, Some(vec!["acbe".to_string(), "acbc".to_string(), "bcbc".to_string()]))),
            TestCase("abcde", "abcde", (0, None)),
            TestCase("afwgebrw", "oarbrenq", (8, Some(vec!["aawgebrw".to_string(), "aargebrw".to_string(), "aarbebrw".to_string(), "aarbebnw".to_string(), "aarbebnq".to_string(), "aarbeenq".to_string(), "aarbrenq".to_string(), "oarbrenq".to_string()]))),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
