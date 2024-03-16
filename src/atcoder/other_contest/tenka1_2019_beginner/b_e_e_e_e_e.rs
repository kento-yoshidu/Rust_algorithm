// https://atcoder.jp/contests/tenka1-2019-beginner/tasks/tenka1_2019_b

pub fn run(_n: usize, s: &str, k: usize) -> String {
    let char = s.chars().nth(k-1).unwrap();

    s.chars()
        .map(|c| {
            if c != char {
                '*'
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "error", 2, "*rr*r"),
            TestCase(6, "eleven", 5, "e*e*e*"),
            TestCase(9, "education", 7, "******i**"),
        ];

        for TestCase(n, s, k, expected) in tests {
            assert_eq!(run(n, s, k), expected);
        }
    }
}
