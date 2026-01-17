// https://yukicoder.me/problems/no/138

fn run(s: &str, t: &str) -> &'static str {
    let a: Vec<u32> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<u32>().unwrap_or(0))
        .collect();

    let b: Vec<u32> = t
        .split(|c: char| !c.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<u32>().unwrap_or(0))
        .collect();

    for (x, y) in a.iter().zip(b) {
        if x > &y {
            return "YES";
        } else if x < &y {
            return "NO";
        }
    }

    "YES"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn yuki_138() {
        let tests = [
            TestCase("4.8.1", "4.8.0", "YES"),
            TestCase("0.0.0", "1.1.1", "NO"),
            TestCase("1.2.3", "3.2.1", "NO"),
            TestCase("11.2.3", "1.2.3", "YES"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
