// https://atcoder.jp/contests/abc151/tasks/abc151_d

use std::collections::VecDeque;

fn check(i: isize, j: isize, h: isize, w: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

fn run(h: usize, w: usize, s: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = 0;

    let dx = [0, -1, 0, 1];
    let dy = [-1, 0, 1, 0];

    // '.'である全ての座標をスタートに設定し、最も大きい距離とansを比べる
    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == '#' {
                continue;
            }

            let mut graph = vec![vec![-1; w]; h];
            graph[i][j] = 0;

            let mut queue = VecDeque::new();
            queue.push_back((i, j));

            while let Some((cur_i, cur_j)) = queue.pop_front() {
                for k in 0..4 {
                    if check(cur_i as isize + dx[k], cur_j as isize + dy[k], h as isize, w as isize) {
                        continue;
                    }

                    let new_i = (cur_i as isize + dx[k]) as usize;
                    let new_j = (cur_j as isize + dy[k]) as usize;

                    if vec[new_i][new_j] == '#' || graph[new_i][new_j] != -1 {
                        continue;
                    }

                    graph[new_i][new_j] = graph[cur_i][cur_j] + 1;
                    ans = ans.max(graph[new_i][new_j] as usize);
                    queue.push_back((new_i, new_j));
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
    fn test() {
        let tests = [
            TestCase(3, 3, vec!["...", "...", "..."], 4),
            TestCase(3, 5, vec!["...#.", ".#.#.", ".#..."], 10),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
