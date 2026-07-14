// https://atcoder.jp/contests/abc314/tasks/abc314_a

fn run(n: usize) -> String {
    let pi: Vec<char> = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".chars().collect();

    pi[0..n+2].into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc314_a() {
        let tests = [
            TestCase(2, "3.14"),
            TestCase(32, "3.14159265358979323846264338327950"),
            TestCase(100, "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
