// https://atcoder.jp/contests/abc400/tasks/abc400_d

use std::collections::VecDeque;

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || i == h as isize || j == w as isize
}

fn run(h: usize, w: usize, s: Vec<&str>, a: usize, b: usize, c: usize, d: usize) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut dist = vec![vec![-1; w]; h];
    dist[a - 1][b - 1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((a - 1, b - 1));

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        for i in 0..4 {
            let new_i = cur_i as isize + dx[i];
            let new_j = cur_j as isize + dy[i];

            if out_of_bounds(h, w, new_i, new_j) {
                continue;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if vec[new_i][new_j] != '#' {
                if dist[new_i][new_j] == -1 || dist[new_i][new_j] > dist[cur_i][cur_j] {
                    dist[new_i][new_j] = dist[cur_i][cur_j];
                    queue.push_front((new_i, new_j));
                }
            } else {
                // 1マス先
                if dist[new_i][new_j] == -1 {
                    dist[new_i][new_j] = dist[cur_i][cur_j] + 1;
                    queue.push_back((new_i, new_j));
                }

                // 2マス先の座標
                let new_i2 = cur_i as isize + dx[i] * 2;
                let new_j2 = cur_j as isize + dy[i] * 2;

                if out_of_bounds(h, w, new_i2, new_j2) {
                    continue;
                }

                let new_i2 = new_i2 as usize;
                let new_j2 = new_j2 as usize;

                if dist[new_i2][new_j2] != -1 {
                    continue;
                }

                dist[new_i2][new_j2] = dist[cur_i][cur_j] + 1;
                queue.push_back((new_i2, new_j2));
            }
        }
    }

    dist[c - 1][d - 1] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize, usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 10, vec!["..........", "#########.", "#.......#.", "#..####.#.", "##....#.#.", "#####.#.#.", ".##.#.#.#.", "###.#.#.#.", "###.#.#.#.", "#.....#..."], 1, 1, 7, 1, 1),
            TestCase(2, 2, vec![".#", "#."], 1, 1, 2, 2, 1),
            TestCase(1, 3, vec![".#."], 1, 1, 1, 3, 1),
            TestCase(20, 20, vec![ "####################", "##...##....###...###", "#.....#.....#.....##", "#..#..#..#..#..#..##", "#..#..#....##..#####", "#.....#.....#..#####", "#.....#..#..#..#..##", "#..#..#.....#.....##", "#..#..#....###...###", "####################", "####################", "##..#..##...###...##", "##..#..#.....#.....#", "##..#..#..#..#..#..#", "##..#..#..#..#..#..#", "##.....#..#..#..#..#", "###....#..#..#..#..#", "#####..#.....#.....#", "#####..##...###...##", "####################"], 3, 3, 18, 18, 3),
        ];

        for TestCase(h, w, s, a, b, c, d, expected) in tests {
            assert_eq!(run(h, w, s, a, b, c, d), expected);
        }
    }
}
