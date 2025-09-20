// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__conditions_branch_2

fn run(n: usize) -> &'static str {
    if n <= 100 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn paiza_d_level_up_05_step02() {
        let tests = [
            TestCase(50, "YES"),
            TestCase(150, "NO"),
            TestCase(100, "YES"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
