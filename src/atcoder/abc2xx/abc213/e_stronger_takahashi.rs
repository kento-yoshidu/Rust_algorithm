// https://atcoder.jp/contests/abc213/tasks/abc213_e

use std::collections::VecDeque;

fn check(i: isize, j: isize, h: isize, w: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

pub fn run(h: usize, w: usize, s: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|str| str.chars().collect()).collect();

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    dist[0][0] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        for i in 0..4 {
            if check(cur_i as isize + dx[i], cur_j as isize + dy[i], h as isize, w as isize) {
                continue;
            }

            let next_i = (cur_i as isize + dx[i]) as usize;
            let next_j = (cur_j as isize + dy[i]) as usize;

            if vec[next_i][next_j] == '#' || dist[next_i][next_j] != std::usize::MAX {
                continue;
            }

            dist[next_i][next_j] = dist[cur_i][cur_j];
            queue.push_front((next_i, next_j));
        }

        for i in -2..=2 {
            for j in -2..=2 {
                let new_i = cur_i as isize + i;
                let new_j = cur_j as isize + j;

                if check(new_i as isize, new_j as isize, h as isize, w as isize) {
                    continue;
                }

                if dist[new_i as usize][new_j as usize] > dist[cur_i][cur_j] + 1 {
                    dist[new_i as usize][new_j as usize] = dist[cur_i][cur_j] + 1;
                    queue.push_back((new_i as usize, new_j as usize));
                }
            }
        }
    }

    for vec in dist {
        for c in vec {
            if c == std::usize::MAX {
                print!("#");
            } else {
                print!("{c}");
            }
        }

        println!();
    }

    let mut ans = 0;

    ans
}