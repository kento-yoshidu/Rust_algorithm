// https://atcoder.jp/contests/abc383/tasks/abc383_c

use std::collections::VecDeque;

fn check(i: isize, j: isize, h: isize, w: isize) -> bool {
    i < 0 || j < 0 || i == h || j == w
}

fn run(h: usize, w: usize, d: usize, s: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut graph = vec![vec![-1; w]; h];
    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == 'H' {
                queue.push_back((i, j));
                graph[i][j] = 0;
            }
        }
    }

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        if d == 0 {
            continue;
        }

        for i in 0..4 {
            let new_i = cur_i as isize + dx[i];
            let new_j = cur_j as isize + dy[i];

            if check(new_i, new_j, h as isize, w as isize) {
                continue;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if vec[new_i][new_j] == '#' || graph[new_i][new_j] != -1 {
                continue;
            }

            graph[new_i][new_j] = graph[cur_i][cur_j] + 1;

            queue.push_back((new_i, new_j));
        }
    }

    graph.into_iter()
        .map(|g| {
            g.into_iter()
                .filter(|i| {
                    *i >= 0 && *i <= d as isize
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, 1, vec!["H...", "#..H", ".#.#"], 5),
            TestCase(5, 6, 2, vec!["##...H", "H.....", "..H.#.", ".HH...", ".###.."], 21),
        ];

        for TestCase(h, w, d, s, expected) in tests {
            assert_eq!(run(h, w, d, s), expected);
        }
    }
}
