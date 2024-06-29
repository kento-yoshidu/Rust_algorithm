// https://atcoder.jp/contests/arc131/tasks/arc131_b

fn run(h: usize, w: usize, arr: Vec<&str>) -> Vec<String> {
    let c: Vec<Vec<char>> = arr.iter().map(|str| str.chars().collect()).collect();

    // 周りにxを敷く
    let mut vec = vec![vec!['x'; w+2]; h+2];

    for i in 0..h {
        for j in 0..w {
            vec[i+1][j+1] = c[i][j];
        }
    }

    // 下、右、上、左
    let dx: Vec<isize>= vec![1, 0, -1, 0];
    let dy= vec![0, 1, 0, -1];

    for i in 1..=h {
        for j in 1..=w {
            if vec[i][j] != '.' {
                continue;
            }

            let mut flag = vec![false; 5];

            for k in 0..4 {
                let x = j as isize + dx[k];
                let y = i as isize + dy[k];

                if let Some(num) = vec[y as usize][x as usize].to_digit(10) {
                    flag[num as usize -1] = true;
                }
            }

            for (idx, b) in flag.iter().enumerate() {
                if *b == false {
                    let c = std::char::from_digit(idx as u32 +1, 10).unwrap();
                    vec[i][j] = c;
                    break;
                }
            }
        }
    }

    let mut ans = Vec::new();

    for i in 1..=h {
        let mut chars = Vec::new();

        for j in 1..=w {
            chars.push(vec[i][j]);
        }

        ans.push(chars.iter().collect::<String>());
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec!["...", "...", "..."], vec!["121", "212", "121"]),
            TestCase(5, 7, vec!["1.2.3.4", ".5.1.2.", "3.4.5.1", ".2.3.4.", "5.1.2.3"], vec!["1324314", "2531423", "3142531", "1253142", "5314213"]),
            TestCase(1, 1, vec!["."], vec!["1"]),
        ];

        for TestCase(h, w, arr, expected) in tests {
            assert_eq!(run(h, w, arr), expected);
        }
    }
}
