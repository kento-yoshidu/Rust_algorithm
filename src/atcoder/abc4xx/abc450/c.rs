// https://atcoder.jp/contests/abc450/tasks/abc450_c

use std::collections::VecDeque;

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || h as isize == i || w as isize == j
}

fn run(h: usize, w: usize, s: Vec<&str>) -> usize {
    let mut s: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = 0;

    let di: [isize; 4] = [0, 1, 0, -1];
    let dj: [isize; 4] = [1, 0, -1, 0];

    for i in 0..h {
        for j in 0..w {
            let mut flag = true;

            let mut queue = VecDeque::new();
            queue.push_front((i, j));

            if s[i][j] == '#' {
                continue;
            }

            s[i][j] = '#';

            while let Some((cur_i, cur_j)) = queue.pop_front() {
                if cur_i == 0 || cur_j == 0 || cur_i == h - 1 || cur_j == w - 1 {
                    flag = false;
                }

                for k in 0..4 {
                    let new_i = cur_i as isize + di[k];
                    let new_j = cur_j as isize + dj[k];

                    if out_of_bounds(h, w, new_i, new_j) {
                        continue;
                    }

                    let new_i = new_i as usize;
                    let new_j = new_j as usize;

                    if s[new_i][new_j] == '.' {
                        s[new_i][new_j] = '#';

                        queue.push_front((new_i, new_j));
                    }
                }
            }

            if flag {
                ans += 1;
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
    fn abc450_c() {
        let tests = [
            TestCase(5, 15, vec!["##########..###", "#...#######.###", "####....###..##", "######.########", "########....###"], 2),
            TestCase(10, 22, vec![ "######################", "####.#################", "###...################", "##.###.##.....########", "##.....##.####.#######", ".######.#......#.....#", ".######.#.####.#.#####", "#########.....##.#####", "################.#####", "################.....#"], 4),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
