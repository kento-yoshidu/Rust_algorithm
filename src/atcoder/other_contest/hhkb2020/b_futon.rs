// https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_b

pub fn run(h: usize, w: usize, vec: Vec<&str>) -> usize {
    let mut ans = 0;

    let map = vec
        .iter()
        .map(|m| m.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for i in 0..w-1 {
        for v in map.iter() {
            if v[i] == '.' && v[i+1] == '.' {
                ans += 1;
            }
        }
    }

    for i in 0..h-1 {
        for j in 0..w {
            if map[i][j] == '.' && map[i+1][j] == '.' {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(2, 3, vec!["..#", "#.."]));
        assert_eq!(0, run(2, 2, vec![".#", "#."]));
    }
}
