// https://yukicoder.me/problems/no/3294

fn run(_n: usize, s: &str) -> String {
    let i = s.chars().position(|c| c == 'c').unwrap();

    format!("UEC{}", &s[i+1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn yuki_3294() {
        let tests = [
            TestCase(9, "yukicoder", "UECoder"),
            TestCase(14, "summervacation", "UECation"),
            TestCase(6, "campus", "UECampus"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
