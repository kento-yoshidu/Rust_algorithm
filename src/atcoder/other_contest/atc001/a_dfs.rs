// https://atcoder.jp/contests/atc001/tasks/dfs_a

fn check(h: isize, w: isize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

fn dfs(vec: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, cur_i: usize, cur_j: usize) {
    seen[cur_i][cur_j] = true;

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    for i in 0..4 {
        if check(vec.len() as isize, vec[0].len() as isize, cur_i as isize + dx[i], cur_j as isize + dy[i]) {
            continue;
        }

        let next_i = (cur_i as isize + dx[i]) as usize;
        let next_j = (cur_j as isize + dy[i]) as usize;

        if vec[next_i][next_j] == '#' || seen[next_i][next_j] {
            continue;
        }

        dfs(&vec, seen, next_i, next_j)
    }
}

fn run(h: usize, w: usize, c: Vec<&str>) -> &'static str {
    let vec: Vec<Vec<char>> = c.into_iter().map(|str| str.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == 's' {
                start = (i, j);
            }
            if vec[i][j] == 'g' {
                end = (i, j);
            }
        }
    }

    let mut seen = vec![vec![false; w]; h];

    dfs(&vec, &mut seen, start.0, start.1);

    if seen[end.0][end.1] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 5, vec!["s####", "....#", "#####", "#...g"], "No"),
            TestCase(4, 4, vec!["...s", "....", "....", ".g.."], "Yes"),
            TestCase(10, 10, vec!["s.........", "#########.", "#.......#.", "#..####.#.", "##....#.#.", "#####.#.#.", "g.#.#.#.#.", "#.#.#.#.#.", "###.#.#.#.", "#.....#..."], "No"),
            TestCase(10, 10, vec!["s.........", "#########.", "#.......#.", "#..####.#.", "##....#.#.", "#####.#.#.", "g.#.#.#.#.", "#.#.#.#.#.", "#.#.#.#.#.", "#.....#..."], "Yes"),
            TestCase(1, 10, vec!["s..####..g"], "No"),
        ];

        for TestCase(h, w, c, expected) in tests {
            assert_eq!(run(h, w, c), expected);
        }
    }
}
