// https://atcoder.jp/contests/arc005/tasks/arc005_1

fn run(_n: usize, w: &str) -> usize {
    let mut vec: Vec<&str> = w.split_whitespace().collect();

    // 末尾の . を取り除く
    if let Some(str) = vec.last_mut() {
        if str.ends_with('.') {
            *str = &str[..str.len() - 1];
        }
    }

    vec.iter()
        .filter(|&&str| {
            str == "TAKAHASHIKUN" || str == "Takahashikun" || str == "takahashikun"
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "Takahashikun is not an eel.", 1),
            TestCase(5, "TAKAHASHIKUN loves TAKAHASHIKUN and takahashikun.", 3),
            TestCase(6, "He is not takahasikun but Takahashikun.", 1),
            TestCase(1, "takahashikunTAKAHASHIKUNtakahashikun.", 0),
            TestCase(18, "You should give Kabayaki to Takahashikun on July twenty seventh if you suspect that he is an eel.", 1),
        ];

        for TestCase(n, w, expected) in tests {
            assert_eq!(run(n, w), expected);
        }
    }
}
