// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__conditions_branch_1

fn run(s: &str) -> &'static str {
    if s == "paiza" {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn paiza_d_level_up_05_step01() {
        let tests = [
            TestCase("paiza", "YES"),
            TestCase("pizza", "NO"),
            TestCase("abc", "NO"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
