// https://atcoder.jp/contests/abc455/tasks/abc455_b

fn run(h: usize, w: usize, s: Vec<&str>) -> usize {
    let s: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = 0;

    for i in 0..h {
        for i2 in i..h {
            for j in 0..w {
                for j2 in j..w {
                    let mut flag = true;

                    for k in i..i2+1 {
                        for l in j..j2+1 {
                            if s[k][l] != s[i+i2-k][j+j2-l] {
                                flag = false
                            }
                        }
                    }

                    if flag {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn abc455_b() {
        let tests = [
            TestCase(3, 2, vec![".#", "#.", "##"], 10),
            TestCase(4, 5, vec![".#.#.", "####.", "##..#", "....#"], 54),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
