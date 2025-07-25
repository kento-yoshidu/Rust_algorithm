// https://atcoder.jp/contests/abc325/tasks/abc325_c

use std::collections::VecDeque;

fn out_of_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

fn run(h: usize, w: usize, s: Vec<&str>) -> usize {
    let mut vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = 0;

    let di = vec![0, 0, -1, 1, -1, -1, 1, 1];
    let dj = vec![-1, 1, 0, 0, -1, 1, -1, 1];

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == '.' {
                continue;
            }

            ans += 1;

            let mut queue = VecDeque::new();
            queue.push_back((i, j));

            while let Some((cur_i, cur_j)) = queue.pop_front() {
                for i in 0..8 {
                    if out_of_bounds(cur_i as isize + di[i], cur_j as isize + dj[i], h as isize, w as isize) {
                        continue;
                    }

                    let new_i = (cur_i as isize + di[i]) as usize;
                    let new_j = (cur_j as isize + dj[i]) as usize;

                    if vec[new_i][new_j] == '#' {
                        vec[new_i][new_j] = '.';
                        queue.push_front((new_i, new_j));
                    }
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
    fn abc325_c() {
        let tests = [
            TestCase(5, 6, vec![".##...", "...#..", "....##", "#.#...", "..#..."], 3),
            TestCase(3, 3, vec!["#.#", ".#.", "#.#"], 1),
            TestCase(4 , 2, vec!["..", "..", "..", ".."], 0),
            TestCase(5, 47, vec![".#..#..#####..#...#..#####..#...#...###...#####", ".#.#...#.......#.#...#......##..#..#...#..#....", ".##....#####....#....#####..#.#.#..#......#####", ".#.#...#........#....#......#..##..#...#..#....", ".#..#..#####....#....#####..#...#...###...#####"], 7),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
