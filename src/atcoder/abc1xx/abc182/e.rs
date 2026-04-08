// https://atcoder.jp/contests/abc182/tasks/abc182_e

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Bulb,
    Block,
}

fn run(h: usize, w: usize, _n: usize, _m: usize, ab: Vec<(usize, usize)>, cd: Vec<(usize, usize)>) -> usize {
    let mut grid = vec![vec![(Cell::Empty, false); w]; h];

    for (a, b) in ab {
        grid[a-1][b-1].0 = Cell::Bulb;
    }

    for (c, d) in cd {
        grid[c-1][d-1].0 = Cell::Block;
    }

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for i in 0..h {
        for j in 0..w {
            if grid[i][j].0 != Cell::Bulb {
                continue;
            }

            grid[i][j].1 = true;

            for (di, dj) in dirs {
                let (mut x, mut y) = (i as i32, j as i32);

                loop {
                    x += di;
                    y += dj;

                    if x < 0 || y < 0 || x >= h as i32 || y >= w as i32 {
                        break;
                    }

                    let (nx, ny) = (x as usize, y as usize);

                    if grid[nx][ny].0 != Cell::Empty {
                        break;
                    }

                    grid[nx][ny].1 = true;
                }
            }
        }
    }

    grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|(_, lit)| *lit)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<(usize, usize)>, Vec<(usize, usize)>, usize);

    #[test]
    fn abc182_e() {
        let tests = [
            TestCase(3, 3, 2, 1, vec![(1, 1), (2, 3)], vec![(2, 2)], 7),
            TestCase(4, 4, 3, 3, vec![(1, 2), (1, 3), (3, 4)], vec![(2, 3), (2, 4), (3, 2)], 8),
            TestCase(5, 5, 5, 1, vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)], vec![(4, 2)], 24),
        ];

        for TestCase(h, w, n, m, ab, cd, expected) in tests {
            assert_eq!(run(h, w, n, m, ab, cd), expected);
        }
    }
}
