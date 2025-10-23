// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_map_step2

fn run(_h: usize, _w: usize, _n: usize, s: Vec<&str>, y: Vec<(usize, usize)>) -> Vec<String> {
    let mut grid: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    for (i, j) in y {
        grid[i][j] = '#';
    }

    grid.into_iter()
        .map(|v| v.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>, Vec<(usize, usize)>, Vec<&'static str>);

    #[test]
    fn paiza_a_level_up_problem_01_step02() {
        let tests = [
            TestCase(3, 3, 1, vec!["...", "...", "..."], vec![(0, 0)], vec!["#..", "...", "..."]),
            TestCase(4, 4, 2, vec!["####", "####", "....", "##.."], vec![(2, 0), (2, 2)], vec!["####", "####", "#.#.", "##.."]),
        ];

        for TestCase(h, w, n, s, y, expected) in tests {
            assert_eq!(run(h, w, n, s, y), expected);
        }
    }
}
