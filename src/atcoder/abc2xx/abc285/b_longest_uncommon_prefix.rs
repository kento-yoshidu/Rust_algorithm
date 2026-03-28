// https://atcoder.jp/contests/abc285/tasks/abc285_b

fn run(n: usize, s: &str) -> Vec<usize> {
    let vec: Vec<char> = s.chars().collect();

    let mut ans = Vec::new();

    for i in 1..n {
        for j in 1..=n {
            if i + j > n {
                ans.push(j-1);
                break;
            }

            if vec[j-1] == vec[i+j-1] {
                ans.push(j-1);
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, Vec<usize>);

    #[test]
    fn abc285_b() {
        let tests = [
            TestCase(6, "abcbac", vec![5, 1, 2, 0, 1]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
