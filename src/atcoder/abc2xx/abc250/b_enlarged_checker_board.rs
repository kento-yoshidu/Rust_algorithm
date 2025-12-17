// https://atcoder.jp/contests/abc250/tasks/abc250_b

fn run(n: usize, a: usize, b: usize) -> Vec<String> {
    let mut ans = vec![vec!['-'; n*b]; n*a];

    for i in 0..n * a {
        for j in 0..n * b {
            let flag = ((i / a) + (j / b)) % 2 == 0;

            if flag {
                ans[i][j] = '.';
            } else {
                ans[i][j] = '#';
            }
        }
    }

    ans.into_iter()
        .map(|a| {
            a.iter().collect::<String>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<&'static str>);

    #[test]
    fn abc250_b() {
        let tests = [
            TestCase(4, 3, 2, vec![ "..##..##",
                                    "..##..##",
                                    "..##..##",
                                    "##..##..",
                                    "##..##..",
                                    "##..##..",
                                    "..##..##",
                                    "..##..##",
                                    "..##..##",
                                    "##..##..",
                                    "##..##..",
                                    "##..##.."]),
            TestCase(5, 1, 5, vec![ ".....#####.....#####.....",
                                    "#####.....#####.....#####",
                                    ".....#####.....#####.....",
                                    "#####.....#####.....#####",
                                    ".....#####.....#####....."]),
            TestCase(4, 4, 1, vec![ ".#.#",
                                    ".#.#",
                                    ".#.#",
                                    ".#.#",
                                    "#.#.",
                                    "#.#.",
                                    "#.#.",
                                    "#.#.",
                                    ".#.#",
                                    ".#.#",
                                    ".#.#",
                                    ".#.#",
                                    "#.#.",
                                    "#.#.",
                                    "#.#.",
                                    "#.#."]),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
