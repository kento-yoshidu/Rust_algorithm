// https://atcoder.jp/contests/abc182/tasks/abc182_c

fn run(_h: usize, w: usize, c: Vec<&str>) -> Vec<usize> {
    (0..w)
        .map(|i| {
            c.iter()
                .filter(|s| {
                    s.chars().nth(i).unwrap() == '#'
                })
                .count()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<usize>);

    #[test]
    fn abc274_b() {
        let tests = [
            TestCase(3, 4, vec!["#..#", ".#.#", ".#.#"], vec![1, 2, 0, 3]),
            TestCase(3, 7, vec![".......", ".......", "......."], vec![0, 0, 0, 0, 0, 0, 0]),
            TestCase(8, 3, vec![".#.", "###", ".#.", ".#.", ".##", "..#", "##.", ".##"], vec![2, 7, 4]),
            TestCase(5, 47, vec![".#..#..#####..#...#..#####..#...#...###...#####", ".#.#...#.......#.#...#......##..#..#...#..#....", ".##....#####....#....#####..#.#.#..#......#####", ".#.#...#........#....#......#..##..#...#..#....", ".#..#..#####....#....#####..#...#...###...#####"], vec![0, 5, 1, 2, 2, 0, 0, 5, 3, 3, 3, 3, 0, 0, 1, 1, 3, 1, 1, 0, 0, 5, 3, 3, 3, 3, 0, 0, 5, 1, 1, 1, 5, 0, 0, 3, 2, 2, 2, 2, 0, 0, 5, 3, 3, 3, 3]),
        ];

        for TestCase(h, w, c, expected) in tests {
            assert_eq!(run(h, w, c), expected);
        }
    }
}
