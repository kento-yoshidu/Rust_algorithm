// https://atcoder.jp/contests/abc364/tasks/abc364_b

fn run(h: usize, w: usize, si: usize, sj: usize, c: Vec<Vec<char>>, x: &str) -> (usize, usize) {
    let mut pos = (si-1, sj-1);

    for r in x.chars() {
        match r {
            'U' => {
                if pos.0 != 0 && c[pos.0-1][pos.1] != '#' {
                    pos = (pos.0 - 1, pos.1);
                }
            },
            'L' => {
                if pos.1 != 0 && c[pos.0][pos.1-1] != '#' {
                    pos = (pos.0, pos.1-1);
                }
            },
            'D' => {
                if pos.0 != h-1 && c[pos.0+1][pos.1] != '#' {
                    pos = (pos.0+1, pos.1);
                }
            },
            'R' => {
                if pos.1 != w-1 && c[pos.0][pos.1+1] != '#' {
                    pos = (pos.0, pos.1+1);
                }
            }
            _ => unreachable!(),
        }
    }

    (pos.0+1, pos.1+1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<Vec<char>>, &'static str, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 2, 1, vec![vec!['.', '#', '.'], vec!['.', '.', '.']], "ULDRU", (2, 2)),
            TestCase(4, 4, 4, 2, vec![vec!['.', '.', '.', '.'], vec!['.', '#', '.', '.'], vec!['.', '.', '.', '#'], vec!['.', '.', '.', '.']], "DUUUURULRD", (2, 4)),
            TestCase(6, 6, 1, 1, vec![vec!['.', '#', '#', '#', '#', '#'], vec!['#', '#', '#', '#', '#', '#'], vec!['#', '#', '#', '#', '#', '#'], vec!['#', '#', '#', '#', '#', '#'], vec!['#', '#', '#', '#', '#', '#'], vec!['#', '#', '#', '#', '#', '#']], "RURLDLULLRULRDL", (1, 1)),
        ];

        for TestCase(h, w, si, sj, c, x, expected) in tests {
            assert_eq!(run(h, w, si, sj, c, x), expected);
        }
    }
}
