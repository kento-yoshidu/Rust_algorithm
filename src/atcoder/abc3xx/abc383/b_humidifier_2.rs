// https://atcoder.jp/contests/abc383/tasks/abc383_b

fn run(h: usize, w: usize, d: isize, s: Vec<&str>) -> usize {
    let chars: Vec<Vec<char>> = s.into_iter().map(|v| v.chars().collect()).collect();

    let mut pos: Vec<(isize, isize)> = Vec::new();

    for i in 0..h {
        for j in 0..w {
            if chars[i][j] != '#' {
                pos.push((i as isize, j as isize));
            }
        }
    }

    let mut ans = 0;

    for i in 0..pos.len() {
        for j in i+1..pos.len() {
            let mut count = 0;
            let mut vec = chars.clone();

            for k in -d..=d {
                for l in -d..=d {
                    if l.abs() + k.abs() <= d {
                        let x1 = pos[i].0 as isize + k;
                        let y1 = pos[i].1 as isize + l;

                        if x1 >= 0 && x1 < h as isize && y1 >= 0 && y1 < w as isize {
                            if vec[x1 as usize][y1 as usize] == '.' {
                                count += 1;
                                vec[x1 as usize][y1 as usize] = '#';
                            }
                        }

                        let x2 = pos[j].0 as isize + k;
                        let y2 = pos[j].1 as isize + l;

                        if x2 >= 0 && x2 < h as isize && y2 >= 0 && y2 < w as isize {
                            if vec[x2 as usize][y2 as usize] == '.' {
                                count += 1;
                                vec[x2 as usize][y2 as usize] = '#';
                            }
                        }
                    }

                }
            }

            ans = ans.max(count);
        }
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, isize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 5, 1, vec![".###.", ".#.##"], 3),
            TestCase(5, 5, 2, vec![".#.#.", ".....", ".#.#.", "#.#.#", "....."], 15),
            TestCase(4, 4, 2, vec!["....", ".##.", ".##.", "...."], 10),
        ];

        for TestCase(h, w, d, s, expected) in tests {
            assert_eq!(run(h, w, d, s), expected);
        }
    }
}
