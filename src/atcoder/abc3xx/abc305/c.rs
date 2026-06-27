// https://atcoder.jp/contests/abc305/tasks/abc305_c

fn run(h: usize, w: usize, s: Vec<&str>) -> (usize, usize) {
    let mut top = h;
    let mut bottom = 0;
    let mut left = w;
    let mut right = 0;

    for (i, row) in s.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' {
                top = top.min(i);
                bottom = bottom.max(i);
                left = left.min(j);
                right = right.max(j);
            }
        }
    }

    let mut ans = (0, 0);

    for i in top..=bottom {
        let row: Vec<_> = s[i].chars().collect();

        for j in left..=right {
            if row[j] != '#' {
                ans = (i+1, j+1);
                break;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, (usize, usize));

    #[test]
    fn abc305_c() {
        let tests = [
            TestCase(5, 6, vec!["......", "..#.#.", "..###.", "..###.", "......"], (2, 4)),
            TestCase(3, 2, vec!["#.", "##", "##"], (1, 2)),
            TestCase(6, 6, vec!["..####", "..##.#", "..####", "..####", "..####", "......"], (2, 5)),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
