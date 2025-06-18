// https://atcoder.jp/contests/arc005/tasks/arc005_3

use std::collections::VecDeque;

fn check(i: isize, j: isize, h: usize, w: usize) -> bool {
    i < 0 || j < 0 || i >= h as isize || j >= w as isize
}

fn run(h: usize, w: usize, c: Vec<&str>) -> &'static str {
    let vec: Vec<Vec<char>> = c.into_iter().map(|s| s.chars().collect()).collect();

    let mut s = (0, 0);
    let mut g = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == 's' {
                s = (i, j);
            }

            if vec[i][j] == 'g' {
                g = (i, j);
            }

        }
    }

    let mut dist = vec![vec![-1; w]; h];
    dist[s.0][s.1] = 0;

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    let mut queue = VecDeque::new();
    queue.push_back((s.0, s.1));

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        for i in 0..4 {
            if check(cur_i as isize + dx[i],cur_j as isize + dy[i], h, w) {
                continue;
            }

            let next_i = (cur_i as isize + dx[i]) as usize;
            let next_j = (cur_j as isize + dy[i]) as usize;

            if dist[next_i][next_j] != -1 {
                continue;
            }

            if vec[next_i][next_j] != '#' {
                dist[next_i][next_j] = dist[cur_i][cur_j];
                queue.push_front((next_i, next_j));
            } else {
                dist[next_i][next_j] = dist[cur_i][cur_j] + 1;
                queue.push_back((next_i, next_j));
            }
        }
    }

    if dist[g.0][g.1] <= 2 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 5, vec!["s####", "....#", "#####", "#...g"], "YES"),
            TestCase(4, 4, vec!["...s", "....", "....", ".g.."], "YES"),
            TestCase(10, 10, vec!["s.........", "#########.", "#.......#.", "#..####.#.", "##....#.#.", "#####.#.#.", "g##.#.#.#.", "###.#.#.#.", "###.#.#.#.", "#.....#..."], "YES"),
            TestCase(6, 6, vec![".....s", "###...", "###...", "######", "...###", "g.####"], "YES"),
            TestCase(1, 10, vec!["s..####..g"], "NO"),
        ];

        for TestCase(h, w, c, expected) in tests {
            assert_eq!(run(h, w, c), expected);
        }
    }
}
