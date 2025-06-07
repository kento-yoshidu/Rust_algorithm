// https://atcoder.jp/contests/abc020/tasks/abc020_c

use std::collections::VecDeque;

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || h as isize == i || w as isize == j
}

fn bfs(h: usize, w: usize, t: usize, s: Vec<&str>, x: isize) -> bool {
    let vec: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    let mut s = (0, 0);
    let mut g = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == 'S' {
                s = (i, j);
            }

            if vec[i][j] == 'G' {
                g = (i, j);
            }
        }
    }

    let mut dist = vec![vec![std::isize::MAX; w]; h];
    dist[s.0][s.1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(s);

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

            let cost = if vec[new_i][new_j] == '#' { x } else { 1 };

            if dist[new_i][new_j] > dist[cur_i][cur_j] + cost {
                dist[new_i][new_j] = dist[cur_i][cur_j] + cost;

                if cost == 1 {
                    queue.push_front((new_i, new_j));
                } else {
                    queue.push_back((new_i, new_j));
                }
            }
        }
    }

    dist[g.0][g.1] <= t as isize
}

fn run(h: usize, w: usize, t: usize, s: Vec<&str>) -> usize {
    let mut low = 1;
    let mut high = 1_000_000_000;
    let mut ans = 1;

    while low <= high {
        let mid = (low + high) / 2;
        if bfs(h, w, t, s.clone(), mid) {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    ans as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>, usize);

    #[test]
    fn abc020_c() {
        let tests = [
            TestCase(2, 3, 10, vec!["S##", ".#G"], 8),
            TestCase(3, 4, 7, vec!["S##G", ".##.", "..#."], 3),
            TestCase(4, 4, 1000000000, vec!["S###", "####", "####", "###G"], 199999999),
        ];

        for TestCase(h, w, t, s, expected) in tests {
            assert_eq!(run(h, w, t, s), expected);
        }
    }
}
