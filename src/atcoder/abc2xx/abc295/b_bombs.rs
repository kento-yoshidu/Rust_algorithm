// https://atcoder.jp/contests/abc295/tasks/abc295_b

pub fn run(r: usize, c: usize, b: Vec<&str>) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = b.iter().map(|s| s.chars().collect()).collect();

    for i in 0..r {
        for j in 0..c {
            if board[i][j] == '.' || board[i][j] == '#' {
                continue
            }

            let dist = (board[i][j] as u8 - b'0') as usize;

            for k in 0..r {
                for l in 0..c {
                    if i.abs_diff(k) + j.abs_diff(l) <= dist && board[k][l] == '#' {
                        board[k][l] = '.';
                    }
                }

                board[i][j] = '.';
            }
        }
    }

    board
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<Vec<char>>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 4, vec![".1.#", "###.", ".#2.", "#.##"], vec![vec!['.', '.', '.', '#'], vec!['#', '.', '.', '.'], vec!['.', '.', '.', '.'], vec!['#', '.', '.', '.']]),
            TestCase(2, 5, vec!["..#.#", "###.#"], vec![vec!['.', '.', '#', '.', '#'], vec!['#', '#', '#', '.', '#']]),
            TestCase(2, 3, vec!["11#", "###"], vec![vec!['.', '.', '.'], vec!['.', '.', '#']]),
            TestCase(4, 6, vec!["#.#3#.", "###.#.", "##.###", "#1..#."], vec![vec!['.', '.', '.', '.', '.', '.'], vec!['#', '.', '.', '.', '.', '.'], vec!['#', '.', '.', '.', '.', '#'], vec!['.', '.', '.', '.', '#', '.']]),
        ];

        for TestCase(r, c, b, expected) in tests {
            assert_eq!(run(r, c, b), expected);
        }
    }
}
