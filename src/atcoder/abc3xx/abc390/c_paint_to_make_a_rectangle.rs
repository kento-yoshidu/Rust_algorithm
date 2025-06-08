// https://atcoder.jp/contests/abc390/tasks/abc390_c

fn run(h: usize, w: usize, s: Vec<&str>) -> &'static str {
    let chars: Vec<Vec<char>> = s.into_iter().map(|str| str.chars().collect()).collect();

    let mut hmin = 1000;
    let mut hmax = 0;
    let mut wmin = 1000;
    let mut wmax = 0;

    for i in 0..h {
        for j in 0..w {
            if chars[i][j] == '#' {
                hmin = hmin.min(i);
                hmax = hmax.max(i);
                wmin = wmin.min(j);
                wmax = wmax.max(j);
            }
        }
    }

    for i in hmin..=hmax {
        for j in wmin..=wmax {
            if chars[i][j] == '.' {
                return "No";
            }
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, vec![".#?#.", ".?#?.", "?...?"], "Yes"),
            TestCase(3, 3, vec!["?##", "#.#", "##?"], "No"),
            TestCase(1, 1, vec!["#"], "Yes"),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
