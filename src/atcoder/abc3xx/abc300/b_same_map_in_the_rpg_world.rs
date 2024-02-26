// https://atcoder.jp/contests/abc300/tasks/abc300_b

pub fn run(h: usize, w: usize, a: Vec<&str>, b: Vec<&str>) -> &'static str {
    let vec_a: Vec<Vec<char>> = a.iter().map(|str| str.chars().collect()).collect();
    let vec_b: Vec<Vec<char>> = b.iter().map(|str| str.chars().collect()).collect();

    for s in 0..h {
        for t in 0..w {
            let mut flag = true;

            for i in 0..h {
                for j in 0..w {
                    if vec_a[(s+i+h)%h][(t+j+w)%w] != vec_b[i][j] {
                        flag = false;
                    }
                }

            }

            if flag == true {
                return "Yes"
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, vec!["..#", "...", ".#.", "..."], vec!["#..", "...", ".#.", "..."], "Yes"),
            TestCase(3, 2, vec!["##", "##", "#."], vec!["..", "#.", "#."], "No"),
            TestCase(4, 5, vec!["#####", ".#...", ".##..", "..##."], vec!["...##", "#...#", "#####", "...#."], "Yes"),
            TestCase(10, 30, vec![  "..........##########..........",
                                    "..........####....###.....##..",
                                    ".....##....##......##...#####.",
                                    "....####...##..#####...##...##",
                                    "...##..##..##......##..##....#",
                                    "#.##....##....##...##..##.....",
                                    "..##....##.##..#####...##...##",
                                    "..###..###..............##.##.",
                                    ".#..####..#..............###..",
                                    "#..........##................."],
                            vec![   "................#..........##.",
                                    "######....................####",
                                    "....###.....##............####",
                                    ".....##...#####......##....##.",
                                    ".#####...##...##....####...##.",
                                    ".....##..##....#...##..##..##.",
                                    "##...##..##.....#.##....##....",
                                    ".#####...##...##..##....##.##.",
                                    "..........##.##...###..###....",
                                    "...........###...#..####..#..."], "Yes"),
        ];

            for TestCase(h, w, a, b, expected) in tests {
                assert_eq!(run(h, w, a, b), expected);
            }
    }
}
