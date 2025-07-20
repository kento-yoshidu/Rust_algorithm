// https://atcoder.jp/contests/abc039/tasks/abc039_d

fn run(h: isize, w: isize, s: Vec<&str>) -> &'static str {
    let vec: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect()).collect();

    let dx = [-1, -1, 0, 1, 1, 1, 0, -1];
    let dy = [0, 1, 1, 1, 0, -1, -1, -1];

    let mut new_vec = vec![vec!['.'; w as usize]; h as usize];

    for i in 0..h {
        for j in 0..w {
            if vec[i as usize][j as usize] == '#' {
                let mut flag = true;

                for k in 0..8 {
                    let y = i as isize + dy[k];
                    let x = j as isize + dx[k];

                    if y >= 0 && x >= 0 && y < h && x < w {
                        if vec[y as usize][x as usize] == '.' {
                            flag = false;
                        }
                    }
                }

                if flag {
                    new_vec[i as usize][j as usize] = '#';
                } else {
                    new_vec[i as usize][j as usize] = '.';
                }
            }
        }
    }

    let mut ans = vec![vec!['.'; w as usize]; h as usize];

    for i in 0..h {
        for j in 0..w {
            if new_vec[i as usize][j as usize] == '#' {
                ans[i as usize][j as usize] = '#';

                for k in 0..8 {
                    let y = i as isize + dy[k];
                    let x = j as isize + dx[k];

                    if y >= 0 && x >= 0 && y < h && x < w {
                        ans[y as usize][x as usize] = '#';
                    }
                }
            }
        }
    }

    if ans == vec {
        "possible"
    } else {
        "impossible"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 4, vec!["##..", "##..", "..##", "..##"], "possible"),
            TestCase(4, 4, vec![ "###.", "####", "..##", "..##"], "possible"),
            TestCase(4, 4, vec!["###.", "##.#", "..##", "..##"], "impossible"),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
