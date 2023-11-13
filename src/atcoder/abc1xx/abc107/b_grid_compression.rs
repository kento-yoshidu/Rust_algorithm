// https://atcoder.jp/contests/abc107/tasks/abc107_b

pub fn run(h: usize, w: usize, a: Vec<&str>) -> Vec<String> {
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

    row_i.iter().map(|r| {
        col_i.iter().map(|c| {
            vec[*r][*c]
        })
        .collect()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["###", "###", ".##"], run(4, 4, vec!["##.#", "....", "##.#", ".#.#"]));
        assert_eq!(vec!["#..", ".#.", "..#"], run(3, 3, vec!["#..", ".#.", "..#"]));
        assert_eq!(vec!["#"], run(4, 5, vec![".....", ".....", "..#..", "....."]));
        assert_eq!(vec!["..#", "#..", ".#.", ".#.", "#.#"], run(7, 6, vec!["......", "....#.", ".#....", "..#...", "..#...", "......", ".#..#."]));
    }
}

