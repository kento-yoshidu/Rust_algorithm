// https://atcoder.jp/contests/agc033/tasks/agc033_a

use std::collections::VecDeque;

// 境界チェック
fn check(r: isize, c: isize, h: usize, w: usize) -> bool {
    r < 0 || c < 0 || r >= h as isize || c >= w as isize
}

fn run(h: usize, w: usize, a: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();

    let mut graph = vec![vec![-1; w]; h];
    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == '#' {
                graph[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    let dx = [0, -1, 0, 1];
    let dy = [-1, 0, 1, 0];

    while let Some((cur_r, cur_c)) = queue.pop_front() {
        for i in 0..4 {
            let new_r = cur_r as isize + dx[i];
            let new_c = cur_c as isize + dy[i];

            if check(new_r, new_c, h, w) {
                continue;
            }

            let (new_r, new_c) = (new_r as usize, new_c as usize);

            if vec[new_r][new_c] == '#' || graph[new_r][new_c] != -1 {
                continue;
            }

            graph[new_r][new_c] = graph[cur_r][cur_c] + 1;
            queue.push_back((new_r, new_c));
        }
    }

    graph.iter()
        .flat_map(|row| row.iter())
        .copied()
        .max()
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec!["...", ".#.", "..."], 2),
            TestCase(6, 6, vec!["..#..#", "......", "#..#..", "......", ".#....", "....#."], 3),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, a), expected);
        }
    }
}
