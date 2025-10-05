// https://atcoder.jp/contests/abc314/tasks/abc314_c

fn run(n: usize, m: usize, s: &str, c: Vec<usize>) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut vec = vec![Vec::new(); m];

    for (idx, i) in c.into_iter().enumerate() {
        vec[i-1].push(idx);
    }

    let mut ans = vec!['.'; n];

    for v in vec {
        let k = v.len();

        if k == 1 {
            ans[v[0]] = chars[v[0]];
            continue;
        }

        for j in 0..k {
            let from = v[(j + k - 1) % k];
            let to = v[j];
            ans[to] = chars[from];
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, Vec<usize>, &'static str);

    #[test]
    fn abc314_c() {
        let tests = [
            TestCase(8, 3,"apzbqrcs", vec![1, 2, 3, 1, 2, 2, 1, 2], "cszapqbr"),
            TestCase(2, 1, "aa", vec![1, 1], "aa"),
        ];

        for TestCase(n, m, s, c, expected) in tests {
            assert_eq!(run(n, m, s, c), expected);
        }
    }
}
