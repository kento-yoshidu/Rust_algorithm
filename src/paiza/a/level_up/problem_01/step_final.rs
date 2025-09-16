// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_map_boss

pub fn run(h: usize, w: usize, s: Vec<&str>) -> Vec<(usize, usize)> {
    let map: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = Vec::new();

    for i in 0..h {
        for j in 0..w {
            // 上下の判定
            let ver =
                if i == 0 {
                    map[i+1][j] == '#'
                } else if i == h-1 {
                    map[i-1][j] == '#'
                } else {
                    map[i+1][j] == '#' && map[i-1][j] == '#'
                };

            // 左右の判定
            let hor =
                if j == 0 {
                    map[i][j+1] == '#'
                } else if j == w-1 {
                    map[i][j-1] == '#'
                } else {
                    map[i][j+1] == '#' && map[i][j-1] == '#'
                };

            if ver && hor {
                ans.push((i, j));
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<(usize, usize)>);

    #[test]
    fn paiza_a_level_up_problem_01_step_final() {
        let tests = [
            TestCase(3, 3, vec!["##.", "###", "..."], vec![(0, 0), (0, 2)]),
            TestCase(10, 10, vec![ "##########", "..........", "##########", "##########", "..........", "#.#.#.#.#.", ".#.#.#.#.#", "#.#.#.#.#.", ".#.#.#.#.#", ".........."], vec![(6, 0), (6, 2), (6, 4), (6, 6), (6, 8), (7, 1), (7, 3), (7, 5), (7, 7), (7, 9)]),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
