// https://paiza.jp/works/mondai/s_rank_skillcheck_sample/search-island

use std::collections::VecDeque;

fn out_of_bounds(h: isize, w: isize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

fn run(n: usize, m: usize, s: Vec<&str>) -> usize {
    let mut map: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let di = [0, 1, 0, -1];
    let dj = [1, 0, -1, 0];

    let mut ans = 0;

    for i in 0..m {
        for j in 0..n {
            if map[i][j] == '0' {
                continue;
            }

            ans += 1;

            let mut queue = VecDeque::new();
            queue.push_back((i, j));

            while let Some((cur_i, cur_j)) = queue.pop_front() {
                for i in 0..4 {
                    let ni = cur_i as isize + di[i];
                    let nj = cur_j as isize + dj[i];

                    if out_of_bounds(m as isize, n as isize, ni, nj) {
                        continue;
                    }

                    let ni = ni as usize;
                    let nj = nj as usize;

                    if map[ni][nj] == '0' {
                        continue;
                    }

                    map[ni][nj] = '0';
                    queue.push_back((ni, nj));
                }
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
    fn paiza_s_skill_check_mihon_problem01() {
        let tests = [
            TestCase(4, 5, vec!["0110", "1010", "1000", "0011", "0111"], 3),
            TestCase(6, 6, vec!["111111", "101000", "101011", "010001", "101111", "010000"], 5),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, s), expected);
        }
    }
}
