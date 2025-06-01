// https://atcoder.jp/contests/abc391/tasks/abc391_b

fn run(n: usize, m: usize, s: Vec<&str>, t: Vec<&str>) -> (usize, usize) {
    let vec_s: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();
    let vec_t: Vec<Vec<char>> = t.into_iter().map(|s| s.chars().collect()).collect();

    for i in 0..=n-m {
        for j in 0..=n-m {
            let mut flag = true;

            for k in 0..m {
                for l in 0..m {
                    if vec_s[i+k][j+l] != vec_t[k][l] {
                        flag = false;
                    }
                }
            }

            if flag {
                return (i+1, j+1);
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec!["#.#", "..#", "##."], vec![".#", "#."], (2, 2)),
            TestCase(2, 1, vec!["#.", "##"], vec!["."], (1, 2)),
        ];

        for TestCase(n, m, s, t, expected) in tests {
            assert_eq!(run(n, m, s, t), expected);
        }
    }
}
