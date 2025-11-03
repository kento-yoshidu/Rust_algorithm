// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_move_step1

fn run(h: usize, w: usize, s: Vec<&str>) -> (usize, usize) {
    let map: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '#' {
                ans = (i, j);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, (usize, usize));

    #[test]
    fn paiza_a_level_up_problem02_step01() {
        let tests = [
            TestCase(1, 1, vec!["#"], (0, 0)),
            TestCase(3, 3, vec![".#.", "...", "..."], (0, 1)),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
