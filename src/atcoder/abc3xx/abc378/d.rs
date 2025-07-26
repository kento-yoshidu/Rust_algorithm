// https://atcoder.jp/contests/abc378/tasks/abc378_d

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || h as isize == i || w as isize == j
}

fn dfs(
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    k: usize,
    step: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    count: &mut usize
) {
    if step == k {
        *count += 1;
        return;
    }

    let di = [0, 1, 0, -1];
    let dj = [1, 0, -1, 0];

    for dir_i in 0..4 {
        let new_i = i as isize + di[dir_i];
        let new_j = j as isize + dj[dir_i];

        if out_of_bounds(h, w, new_i, new_j) {
            continue;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;

        if visited[new_i][new_j] || grid[new_i][new_j] == '#' {
            continue;
        }

        visited[new_i][new_j] = true;
        dfs(h, w, new_i, new_j, k, step+1, &grid, visited, count);
        visited[new_i][new_j] = false;

    }
}

fn run(h: usize, w: usize, k: usize, s: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut visited = vec![vec![false; w]; h];

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if vec[i][j] == '#' {
                continue;
            }

            visited[i][j] = true;
            dfs(h, w, i, j, k, 0, &vec, &mut visited, &mut ans);
            visited[i][j] = false;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 2, vec![".#", ".."], 2),
            TestCase(2, 3, 1, vec![".#.", "#.#"], 0),
            TestCase(10, 10, 11, vec!["....#..#..", ".#.....##.", "..#...##..", "...#......", "......##..", "..#......#", "#........#", "..##......", ".###....#.", "...#.....#"], 218070),
        ];

        for TestCase(h, w, k, s, expected) in tests {
            assert_eq!(run(h, w, k, s), expected);
        }
    }
}
