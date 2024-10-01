// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_c

fn run(n: usize, s: &str) -> String {
    let mut ans: Vec<char> = s.chars().collect();

    for i in 0..n-1 {
        if ans[i] == ans[i+1] {
            let c = ans[i].to_uppercase().next().unwrap();
            ans[i] = c;
            ans[i+1] = c;
        }
    }

    ans.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "jjoiii", "JJoIIi"),
            TestCase(6, "joijoi", "joijoi"),
            TestCase(7, "ooooooo", "OOOOOOo"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }

}
