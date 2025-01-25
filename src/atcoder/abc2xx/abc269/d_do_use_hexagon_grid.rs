// https://atcoder.jp/contests/abc269/tasks/abc269_d

use std::collections::VecDeque;

fn check(r: isize, c: isize) -> bool {
    r < 0 || c < 0 || r >= 2001 || c >= 2001
}

fn run(_n: usize, xy: Vec<(isize, isize)>) -> usize {
    let offset = 1000;

    let xy: Vec<(isize, isize)> = xy.into_iter().map(|(x, y)| (x+offset, y+offset)).collect();

    let mut vec = vec![vec![false; 2001]; 2001];

    for (x, y) in xy.iter() {
        vec[*x as usize][*y as usize] = true;
    }

    let dx = [0, 1, 1, 0, -1, -1];
    let dy = [1, 1, 0, -1, -1, 0];

    let mut ans = 0;

    for (x, y) in xy.iter() {
        if !vec[*x as usize][*y as usize] {
            continue;
        }

        ans += 1;

        let mut queue = VecDeque::new();

        queue.push_back((*x, *y));

        while let Some((cur_i, cur_j)) = queue.pop_front() {
            if !vec[cur_i as usize][cur_j as usize] {
                continue;
            }

            vec[cur_i as usize][cur_j as usize] = false;

            for i in 0..6 {
                if check(cur_i as isize + dx[i], cur_j as isize + dy[i]) {
                    continue;
                }

                let new_i = (cur_i as isize + dx[i]) as usize;
                let new_j = (cur_j as isize + dy[i]) as usize;

                if vec[new_i][new_j] {
                    queue.push_back((new_i as isize, new_j as isize));
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![(-1, -1), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0)], 3),
            TestCase(4, vec![(5, 0), (4, 1), (-3, 4), (-2, -5)], 4),
            TestCase(5, vec![(2, 1), (2, -1), (1, 0), (3, 1), (1, -1)], 1),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
