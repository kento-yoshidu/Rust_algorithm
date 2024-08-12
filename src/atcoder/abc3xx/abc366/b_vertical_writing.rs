// https://atcoder.jp/contests/abc366/tasks/abc366_b

pub fn run(n: usize, s: Vec<&str>) -> Vec<String> {
    let num = s.iter()
        .map(|str| str.len())
        .max()
        .unwrap();

    let mut ans = vec![vec!['*'; n]; num];

    for i in (0..n).rev() {
        for (j, c) in s[i].chars().enumerate() {
            ans[j][n - i - 1] = c;
        }
    }

    for i in 0..num {
        while *ans[i].last().unwrap() == '*' {
            ans[i].pop().unwrap();
        }
    }

    ans.into_iter()
        .map(|vec| vec.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["abc", "de", "fghi"], vec!["fda", "geb", "h*c", "i"]),
            TestCase(3, vec!["atcoder", "beginner", "contest"], vec!["cba", "oet", "ngc", "tio", "end", "sne", "ter", "*r"]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}