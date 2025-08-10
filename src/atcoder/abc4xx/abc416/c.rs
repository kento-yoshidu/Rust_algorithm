// https://atcoder.jp/contests/abc416/tasks/abc416_c

fn dfs(current: String, count: usize, s: &Vec<&str>, result: &mut Vec<String>) {
    if count == 0 {
        result.push(current);
    } else {
        for &ch in s.iter() {
            dfs(format!("{}{}", current, ch), count - 1, s, result);
        }
    }
}

fn run(_n: usize, k: usize, x: usize, s: Vec<&str>) -> String {
    let mut vec = Vec::new();

    dfs(String::new(), k, &s, &mut vec);

    vec.sort();

    vec[x-1].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc416_c() {
        let tests = [
            TestCase(3, 2, 6, vec!["abc", "xxx", "abc"], "abcxxx"),
            TestCase(5, 5, 416, vec!["a", "aa", "aaa", "aa", "a"], "aaaaaaa"),
        ];

        for TestCase(n, k, x, s, expected) in tests {
            assert_eq!(run(n, k, x, s), expected);
        }
    }
}
