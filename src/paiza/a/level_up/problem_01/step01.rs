// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_map_step1

fn run(_h: usize, _w: usize, _n: usize, s: Vec<&str>, y: Vec<(usize, usize)>) -> Vec<char> {
    let grid: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    y.into_iter()
        .map(|(i, j)| grid[i][j])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>, Vec<(usize, usize)>, Vec<char>);

    #[test]
    fn paiza_a_level_up_problem_01_step01() {
        let tests = [
            TestCase(3, 3, 2, vec!["###", "###", "..."], vec![(2, 2), (1, 1)], vec!['.', '#']),
            TestCase(2, 2, 1, vec!["#.", ".#"], vec![(0, 1)], vec!['.']),
        ];

        for TestCase(h, w, n, s, y, expected) in tests {
            assert_eq!(run(h, w, n, s, y), expected);
        }
    }
}
