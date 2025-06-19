// https://atcoder.jp/contests/abc395/tasks/abc395_b

fn run(n: usize) -> Vec<String> {
    let mut ans = vec![vec!['.'; n]; n];

    for i in 0..n {
        let j = n + 1 - i-1;

        if i > j {
            continue;
        }

        if i % 2 == 0 {
            for k in i..j {
                for l in i..j {
                    ans[k][l] = '#';
                }
            }
        } else {
            for k in i..j {
                for l in i..j {
                    ans[k][l] = '.';
                }
            }
        }
    }

    ans.into_iter()
        .map(|vec| vec.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(11, vec!["###########", "#.........#", "#.#######.#", "#.#.....#.#", "#.#.###.#.#", "#.#.#.#.#.#", "#.#.###.#.#", "#.#.....#.#", "#.#######.#", "#.........#", "###########"]),
            TestCase(5, vec!["#####", "#...#", "#.#.#", "#...#", "#####"]),
            TestCase(8, vec!["########", "#......#", "#.####.#", "#.#..#.#", "#.#..#.#", "#.####.#", "#......#", "########"]),
            TestCase(2, vec!["##", "##"]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
