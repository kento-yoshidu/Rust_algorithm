// https://atcoder.jp/contests/abc341/tasks/abc341_c

use std::collections::HashSet;

pub fn run(_h: usize, _w: usize, _n: usize, t: &str, s: Vec<&str>) -> usize {
    let hash_set: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    let mut ans = HashSet::new();

    for (i, vec) in s.iter().enumerate() {
        'outer: for (j, c) in vec.chars().enumerate() {
            // 陸'.'からスタート
            if c == '.' {
                // 最初の座標
                let mut pos: (isize, isize) = (i as isize, j as isize);

                for d in t.chars() {
                    match d {
                        'L'=> pos.1 -= 1,
                        'R' => pos.1 += 1,
                        'U' => pos.0 -= 1,
                        'D' => pos.0 += 1,
                        _ => unreachable!(),
                    }

                    // 海'#'に到着したら飛ばす
                    if hash_set[pos.0 as usize][pos.1 as usize] == '#' {
                        continue 'outer;
                    }
                }

                ans.insert(pos);
            }
        }
    }

    ans.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 7, 5, "LULDR", vec!["#######", "#...#.#", "##...##", "#.#...#", "#...#.#", "#######"], 2),
            TestCase(13, 16, 9, "ULURDLURD", vec![ "################", "##..##.#..####.#", "###.#..#.....#.#", "#..##..#####.###", "#...#..#......##", "###.##.#..#....#", "##.#####....##.#", "###.###.#.#.#..#", "######.....##..#", "#...#.#.######.#", "##..###..#..#.##", "#...#.#.#...#..#", "################"], 6),
        ];

        for TestCase(h, w, n, t, s, expected) in tests {
            assert_eq!(run(h, w, n, t ,s), expected);
        }
    }
}
