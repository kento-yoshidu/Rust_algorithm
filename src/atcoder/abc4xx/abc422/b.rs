// https://atcoder.jp/contests/abc422/tasks/abc422_b

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || h as isize <= i || w as isize <= j
}

fn run(h: usize, w: usize, s: Vec<&str>) -> &'static str {
    let chars: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    for i in 0..h {
        for j in 0..w {
            if chars[i][j] == '.' {
                continue;
            }

            let mut count = 0;

            let di = [0, 1, 0, -1];
            let dj = [1, 0, -1, 0];

            for k in 0..4 {
                if out_of_bounds(h, w, i as isize + di[k], j as isize +dj[k]){
                    continue;
                }

                let ni = (i as isize + di[k]) as usize;
                let nj = (j as isize + dj[k]) as usize;

                if chars[ni][nj] == '#' {
                    count += 1;
                }
            }

            if count != 2 && count != 4 {
                return "No";
            }
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc422_b() {
        let tests = [
            TestCase(8, 7, vec![".######", "##....#", "#.###.#", "#.#.#.#", "#.#.#.#", "#.#####", "#...#..", "#####.."], "Yes"),
            TestCase(1, 2, vec!["##"], "No"),
            TestCase(4, 3, vec!["...", "...", "...", "..."], "Yes"),
            TestCase(15, 18, vec!["##.###..##.##..##.", "##.#.##.##.##.####", "...##.#.......####", "###.###....###.##.", "#.##.......#.#....", "#..#.##.##.#.#....", "#.########.####.##", "#.##.##.#....##.##", "#......##.........", "##.##..#..##..####", ".#.#####..#####..#", ".#..#...##.#.....#", ".#..#.####.#.....#", ".##.#.#.#..##..###", "..###.###...####.."], "Yes"),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
