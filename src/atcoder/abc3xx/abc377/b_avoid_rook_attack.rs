// https://atcoder.jp/contests/abc377/tasks/abc377_b

fn run(s: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|v| v.chars().collect()).collect();

    let h = vec.iter()
        .filter(|v| {
            v.contains(&('#'))
        })
        .count();

    let v = (0..8)
        .filter(|i| {
            vec.iter().any(|v| {
                v[*i] == '#'
            })
        })
        .count();

    (8 - h) * (8 - v)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec!["...#....", "#.......", ".......#", "....#...", ".#......", "........", "........", "..#....."], 4),
            TestCase(vec!["........",  "........", "........", "........", "........", "........", "........", "........"], 64),
            TestCase(vec![".#......", "..#..#..", "....#...", "........", "..#....#", "........", "...#....", "....#..."], 4),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
