// https://atcoder.jp/contests/abc176/tasks/abc176_d

use std::collections::VecDeque;

fn out_of_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

const INF: isize = std::isize::MAX;

fn run(h: usize, w: usize, c: (usize, usize), d: (usize, usize), s: Vec<&str>) -> isize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut dist = vec![vec![INF; w]; h];
    dist[c.0 - 1][c.1 - 1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((c.0-1, c.1-1));

    let di = [0, 1, 0, -1];
    let dj = [1, 0, -1, 0];

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        for i in 0..4 {
            if out_of_bounds(cur_i as isize + di[i], cur_j as isize + dj[i], h as isize, w as isize) {
                continue;
            }

            let next_i = (cur_i as isize + di[i]) as usize;
            let next_j = (cur_j as isize + dj[i]) as usize;

            if vec[next_i][next_j] == '#' || dist[next_i][next_j] <= dist[cur_i][cur_j] {
                continue;
            }

            dist[next_i][next_j] = dist[cur_i][cur_j];
            queue.push_front((next_i, next_j));
        }

        for i in -2..=2 {
            for j in -2..=2 {
                let new_i = cur_i as isize + i;
                let new_j = cur_j as isize + j;

                if out_of_bounds(cur_i as isize + i, cur_j as isize + j, h as isize, w as isize) {
                    continue;
                }

                let jump_i = new_i as usize;
                let jump_j = new_j as usize;

                if vec[jump_i][jump_j] == '#' || dist[jump_i][jump_j] <= dist[cur_i][cur_j] + 1 {
                    continue;
                }

                dist[jump_i][jump_j] = dist[cur_i][cur_j] + 1;
                queue.push_back((jump_i, jump_j));
            }
        }
    }

    if dist[d.0-1][d.1-1] == INF {
        -1
    } else {
        dist[d.0-1][d.1-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, (usize, usize), (usize, usize), Vec<&'static str>, isize);

    #[test]
    fn abc176_d() {
        let tests = [
            TestCase(4, 4, (1, 1), (4, 4), vec!["..#.", "..#.", ".#..", ".#.."], 1),
            TestCase(4, 4, (1, 4), (4, 1), vec![".##.", "####", "####", ".##."], -1),
            TestCase(4, 4, (2, 2), (3, 3), vec!["....", "....", "....", "...."], 0),
            TestCase(4, 5, (1, 2), (2, 5), vec!["#.###", "####.", "#..##", "#..##"], 2),
        ];

        for TestCase(h, w, c, d, s, expected) in tests {
            assert_eq!(run(h, w, c, d, s), expected);
        }
    }
}
