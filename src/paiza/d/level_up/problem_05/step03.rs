// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__conditions_branch_3

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a * b <= c {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn paiza_d_level_up_05_step03() {
        let tests = [
            TestCase(2, 3, 7, "YES"),
            TestCase(3, 6, 17, "NO"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
