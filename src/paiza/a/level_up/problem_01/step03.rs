// https://paiza.jp/works/mondai/a_rank_level_up_problems/a_rank_snake_map_step3

fn run(h: usize, w: usize, s: Vec<&str>) -> Vec<(usize, usize)> {
    let map: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = Vec::new();

    for i in 0..h {
        for j in 0..w {
            if j == 0 {
                if map[i][j+1] == '#' {
                    ans.push((i, j));
                }

                continue;
            }

            if j == w-1 {
                if map[i][j-1] == '#' {
                    ans.push((i, j));
                }

                continue;
            }

            if map[i][j-1] == '#' && map[i][j+1] == '#' {
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
    fn paiza_a_level_up_problem_01_step03() {
        let tests = [
            TestCase(3, 3, vec![ "#.#", ".#.", "..."], vec![(0, 1), (1, 0), (1, 2)]),
            TestCase(4, 4, vec!["####", "####", "####", "####"], vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (1, 3), (2, 0), (2, 1), (2, 2), (2, 3), (3, 0), (3, 1), (3, 2), (3, 3)]),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
