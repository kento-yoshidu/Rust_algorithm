// https://atcoder.jp/contests/abc404/tasks/abc404_b

fn count_diff(n: usize, s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[i][j] {
                count += 1;
            }
        }
    }

    count
}

fn rotate(grid: Vec<Vec<char>>, times: usize) -> Vec<Vec<char>> {
    let n = grid.len();
    let mut result = grid;

    for _ in 0..(times % 4) {
        let mut new_grid = vec![vec![' '; n]; n];

        for i in 0..n {
            for j in 0..n {
                new_grid[j][n - 1- i] = result[i][j];
            }
        }

        result = new_grid;
    }

    result
}

fn run(n: usize, s: Vec<&str>, t: Vec<&str>) -> usize {
    let s: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();
    let t: Vec<Vec<char>> = t.into_iter().map(|s| s.chars().collect()).collect();

    let mut ans = n * n;

    for i in 0..4 {
        let s = rotate(s.clone(), i);

        ans = ans.min(count_diff(n, &s, &t) + i);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec!["###.", "..#.", "..#.", "..#."], vec!["...#", "...#", "###.", "...."], 2),
            TestCase(13, vec![".#..###..##..", "#.#.#..#.#.#.", "#.#.###..#...", "###.#..#.#.#.", "#.#.###..##..", ".............", "..#...#....#.", ".##..#.#..##.", "#.#..#.#.#.#.", "####.#.#.####", "..#..#.#...#.", "..#...#....#.", "............."], vec![".............", ".#....#...#..", ".#...#.#..#..", "####.#.#.####", ".#.#.###..#.#", ".##....#..##.", ".#....#...#..", ".............", "..##..###.#.#", ".#.#.#..#.###", ".#.#..###.#.#", ".#.#.#..#.#.#", "..##..###..#."], 5),
        ];

        for TestCase(n, s, t, expected) in tests {
            assert_eq!(run(n, s, t), expected);
        }
    }
}
