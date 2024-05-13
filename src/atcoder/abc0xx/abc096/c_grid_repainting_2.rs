// https://atcoder.jp/contests/abc096/tasks/abc096_c

pub fn run(h: usize, w: usize, s: Vec<&str>) -> &'static str {
    let mut grid: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    // 範囲外アクセスを防ぐため、周りをxで囲う
    grid.insert(0, vec!['x'; w+2]);
    grid.push(vec!['x'; w+2]);

    for i in 1..=h {
        grid[i].insert(0, 'x');
        grid[i].push('x');
    }

    for i in 1..=h {
        for j in 1..=w {
            if grid[i][j] == '#' {
                if grid[i][j-1] != '#'
                && grid[i-1][j] != '#'
                && grid[i+1][j] != '#'
                && grid[i][j+1] != '#' {
                    return "No";
                }
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
    fn test() {
        let tests = [
            TestCase(3, 3, vec![".#.", "###", ".#."], "Yes"),
            TestCase(5, 5, vec!["#.#.#", ".#.#.", "#.#.#", ".#.#.", "#.#.#"], "No"),
            TestCase(11, 11, vec!["...#####...",".##.....##.","#..##.##..#","#..##.##..#","#.........#","#...###...#",".#########.",".#.#.#.#.#.","##.#.#.#.##","..##.#.##..",".##..#..##."], "Yes"),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
