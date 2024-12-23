// https://atcoder.jp/contests/abc385/tasks/abc385_b

use std::collections::HashSet;

fn run(_h: usize, _w: usize, x: usize, y: usize, s: Vec<&str>, t: &str) -> (usize, usize, usize) {
    let mut vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut count = 0;

    if vec[x-1][y-1] == '@' {
        count += 1;
    }

    let (pos, count) = t.chars()
        .fold(((x-1, y-1), count), |(mut pos, mut count), c| {
            match c {
                'L' => {
                    if vec[pos.0][pos.1-1] != '#' {
                        pos.1 -= 1;
                    }
                }
                'R' => {
                    if vec[pos.0][pos.1+1] != '#' {
                        pos.1 += 1;
                    }
                }
                'U' => {
                    if vec[pos.0-1][pos.1] != '#' {
                        pos.0 -= 1;
                    }
                }
                'D' => {
                    if vec[pos.0+1][pos.1] != '#' {
                        pos.0 += 1;
                    }
                }
                _ => unreachable!(),
            }

            if vec[pos.0][pos.1] == '@' {
                count += 1;
                vec[pos.0][pos.1] = '.';
            }

        (pos, count)
    });

    (pos.0 + 1, pos.1 + 1, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<&'static str>, &'static str, (usize, usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 5, 3, 4, vec!["#####", "#...#", "#.@.#", "#..@#", "#####"], "LLLDRUU", (2, 3, 1)),
            TestCase(6, 13, 4, 6, vec!["#############", "#@@@@@@@@@@@#", "#@@@@@@@@@@@#", "#@@@@.@@@@@@#", "#@@@@@@@@@@@#", "#############"], "UURUURLRLUUDDURDURRR", (3, 11, 11)),
            TestCase(12, 35, 7, 10, vec![ "###################################", "#.................................#", "#..........@......................#", "#......@................@.........#", "#.............##............@.....#", "#...##........##....##............#", "#...##........##....##.......##...#", "#....##......##......##....##.....#", "#....##......##......##..##.......#", "#.....#######.........###.........#", "#.................................#", "###################################"], "LRURRRUUDDULUDUUDLRLRDRRLULRRUDLDRU", (4, 14, 1)),
        ];

        for TestCase(h, w, x, y, s, t, expected) in tests {
            assert_eq!(run(h, w, x, y, s, t), expected);
        }
    }
}
