// https://atcoder.jp/contests/abc276/tasks/abc276_e

use std::collections::VecDeque;

fn check(i: isize, j: isize, h: isize, w: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

fn run(h: usize, w: usize, c: Vec<&str>) -> &'static str {
    let vec: Vec<Vec<char>> = c.into_iter().map(|str| str.chars().collect()).collect();

    let mut s_pos = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == 'S' {
                s_pos = (i as isize, j as isize);
            }
        }
    }

    // スタートとゴールの組み合わせ
    let pair = [
        ((-1, 0), (1, 0)), // 下上
        ((-1, 0), (0, 1)), // 下右
        ((-1, 0), (0, -1)), // 下左
        ((0, 1), (0, -1)), // 右左
        ((1, 0), (0, 1)), // 下右
        ((1, 0), (0, -1)), // 下左
        ];

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    for i in 0..6 {
        let start_i = s_pos.0 + pair[i].0.0;
        let start_j = s_pos.1 + pair[i].0.1;

        let end_i = s_pos.0 + pair[i].1.0;
        let end_j = s_pos.1 + pair[i].1.1;

        if check(start_i, start_j, h as isize, w as isize) || check(end_i, end_j, h as isize, w as isize) {
            continue;
        }

        if vec[start_i as usize][start_j as usize] == '#' || vec[end_i as usize][end_j as usize] == '#' {
            continue;
        }

        let start = (start_i as usize, start_j as usize);

        let mut dist = vec![vec![-1; w]; h];
        dist[s_pos.0 as usize][s_pos.1 as usize] = 0;
        dist[start.0][start.1] = 1;

        let mut queue = VecDeque::new();
        queue.push_front((start.0, start.1));

        while let Some((cur_i, cur_j)) = queue.pop_front() {
            for i in 0..4 {
                if check(cur_i as isize + dx[i], cur_j as isize + dy[i], h as isize, w as isize) {
                    continue;
                }

                let next_i = (cur_i as isize + dx[i]) as usize;
                let next_j = (cur_j as isize + dy[i]) as usize;

                if vec[next_i][next_j] == '#' || dist[next_i][next_j] != -1 {
                    continue;
                }

                dist[next_i][next_j] = dist[cur_i][cur_j] + 1;
                queue.push_back((next_i, next_j));
            }
        }

        if dist[end_i as usize][end_j as usize] >= 3 {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 4, vec!["....", "#.#.", ".S..", ".##."], "Yes"),
            TestCase(2, 2, vec!["S.", ".#"], "No"),
            TestCase(5, 7, vec![".#...#.", "..#.#..", "...S...", "..#.#..", ".#...#."], "No"),
            TestCase(4, 4, vec!["...S", "....", "....", "...."], "Yes"),
        ];

        for TestCase(h, w, c, expected) in tests {
            assert_eq!(run(h, w, c), expected);
        }
    }
}
