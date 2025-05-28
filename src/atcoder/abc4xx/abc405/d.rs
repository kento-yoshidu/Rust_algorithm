// https://atcoder.jp/contests/abc405/tasks/abc405_d

use std::collections::VecDeque;

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || h as isize == i || w as isize == j
}

fn run(h: usize, w: usize, s: Vec<&str>) -> Vec<String> {
    let mut vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == 'E' {
                queue.push_back((i, j));
            }
        }
    }

    let di = [0, 1, 0, -1];
    let dj = [1, 0, -1, 0];
    let dis = ['<', '^', '>', 'v'];

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        for i in 0..4 {
            let new_i = cur_i as isize + di[i];
            let new_j = cur_j as isize + dj[i];

            if out_of_bounds(h, w, new_i, new_j) {
                continue;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if vec[new_i][new_j] != '.' {
                continue;
            }

            vec[new_i][new_j] = dis[i];

            queue.push_back((new_i, new_j));
        }
    }

    vec.into_iter()
        .map(|v| v.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc405_d() {
        let tests = [
            TestCase(3, 4, vec!["...E", ".#..", "...."], vec![">>>E", "^#>^", ">>>^"]),
            TestCase(3, 2, vec!["##", "##", "##"], vec!["##", "##", "##"]),
            TestCase(7, 20, vec!["....................", "..#..#..####..#E##..", "..#..#..#..#..#.....", "..E###..#..#..####..", ".....#..#..E.....#..", ".....#..####..####..", "...................."], vec!["vv<<<<<>>>>>>>>v<<<<", "vv#^^#^^####v^#E##vv", "vv#^^#v^#vv#vv#^<<<<", ">>E###vv#vv#vv####^^", ">>^<<#vv#>>E<<<<<#^^", ">>^^^#vv####^^####^^", ">>^^^<<<>>>>^^<<<<^^"]),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
