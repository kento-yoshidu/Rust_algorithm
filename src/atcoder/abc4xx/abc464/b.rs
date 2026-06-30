// https://atcoder.jp/contests/abc464/tasks/abc464_b

fn run(h: usize, w: usize, c: Vec<&str>) -> Vec<String> {
    let mut min_row = h;
    let mut max_row = 0;
    let mut min_col = w;
    let mut max_col = 0;

    for i in 0..h {
        for (j, ch) in c[i].chars().enumerate() {
            if ch == '#' {
                min_row = min_row.min(i);
                max_row = max_row.max(i);
                min_col = min_col.min(j);
                max_col = max_col.max(j);
            }
        }
    }

    c[min_row..=max_row]
        .iter()
        .map(|row| row[min_col..=max_col].to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc464_b() {
        let tests = [
            TestCase(4, 5, vec![".....", "..#..", ".###.", "....."], vec![".#.", "###"]),
            TestCase(3, 4, vec!["#...", "....", "...#"], vec!["#...", "....", "...#"]),
            TestCase(5, 6, vec!["......", "......", "...#..", "......", "......"], vec!["#"]),
        ];

        for TestCase(h, w, c, expected) in tests {
            assert_eq!(run(h, w, c), expected);
        }
    }
}
