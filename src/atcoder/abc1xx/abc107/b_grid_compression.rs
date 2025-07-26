// https://atcoder.jp/contests/abc107/tasks/abc107_b

fn run(h: usize, w: usize, a: Vec<&str>) -> Vec<String> {
    let vec: Vec<Vec<char>> = a.iter().map(|str| str.chars().collect()).collect();

    // 何行目を残すか
    let mut row_i = Vec::new();

    (0..h).for_each(|col| {
        if (0..w).any(|row| {
            vec[col][row] == '#'
        }) {
            row_i.push(col)
        }
    });

    // 何列目を残すか
    let mut col_i = Vec::new();

    (0..w).for_each(|row| {
        if (0..h).any(|col| {
            vec[col][row] == '#'
        }) {
            col_i.push(row)
        }
    });

    row_i.into_iter().map(|r| {
        col_i.iter().map(|c| {
            vec[r][*c]
        })
        .collect()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc107_b() {
        let tests = [
            TestCase(4, 4, vec!["##.#", "....", "##.#", ".#.#"], vec!["###", "###", ".##"]),
            TestCase(3, 3, vec!["#..", ".#.", "..#"], vec!["#..", ".#.", "..#"]),
            TestCase(4, 5, vec![".....", ".....", "..#..", "....."], vec!["#"]),
            TestCase(7, 6, vec!["......", "....#.", ".#....", "..#...", "..#...", "......", ".#..#."], vec!["..#", "#..", ".#.", ".#.", "#.#"]),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, a), expected);
        }
    }
}

