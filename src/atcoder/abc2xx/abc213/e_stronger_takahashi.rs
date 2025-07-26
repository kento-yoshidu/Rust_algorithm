// https://atcoder.jp/contests/abc213/tasks/abc213_e

use std::collections::VecDeque;

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || h as isize <= i || w as isize <= j
}

fn run(h: usize, w: usize, s: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|str| str.chars().collect()).collect();

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    dist[0][0] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    let di = [0, 1, 0, -1];
    let dj = [1, 0, -1, 0];

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        for i in 0..4 {
            let new_i = cur_i as isize + di[i];
            let new_j = cur_j as isize + dj[i];

            if out_of_bounds(h, w, new_i, new_j) {
                continue;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if vec[new_i][new_j] == '#' {
                continue;
            }

            if dist[new_i][new_j] > dist[cur_i][cur_j] {
                dist[new_i][new_j] = dist[cur_i][cur_j];
                queue.push_front((new_i, new_j));
            }
        }

        for di in -2..=2 {
            for dj in -2..=2 {
                let new_i = cur_i as isize + di;
                let new_j = cur_j as isize + dj;

                if di.abs() + dj.abs() == 4 {
                    continue;
                }

                if out_of_bounds(h, w, new_i, new_j) {
                    continue;
                }

                let new_i = new_i as usize;
                let new_j = new_j as usize;

                if dist[new_i][new_j] > dist[cur_i][cur_j] + 1 {
                    dist[new_i][new_j] = dist[cur_i][cur_j] + 1;
                    queue.push_back((new_i, new_j));
                }
            }
        }
    }

    dist[h-1][w-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn abc213_e() {
        let tests = [
            TestCase(5, 5, vec!["..#..", "#.#.#", "##.##", "#.#.#", "..#.."], 1),
            TestCase(5, 7, vec![".......", "######.", ".......", ".######", "......."], 0),
            TestCase(8, 8, vec![".#######", "########", "########", "########", "########", "########", "########", "#######."], 5),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
