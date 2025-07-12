// https://atcoder.jp/contests/joi2020yo1c/tasks/joi2020_yo1c_b

fn run(n: usize, s: &str) -> String {
    let mut s: Vec<char> = s.chars().collect();

    for i in 0..n-2 {
        if s[i] == 'j' && s[i+1] == 'o' && s[i+2] == 'i' {
            s[i] = 'J';
            s[i+1] = 'O';
            s[i+2] = 'I';
        }
    }

    s.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(11, "joinojoijin", "JOInoJOIjin"),
            TestCase(16, "jjooiiijoiojioij", "jjooiiiJOIojioij"),
            TestCase(13, "nihongoutenai", "nihongoutenai"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
