// https://atcoder.jp/contests/abc075/tasks/abc075_b

pub fn run(h: isize, w: isize, s: Vec<&str>) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect::<Vec<char>>()).collect();

    // 8方向の内、どの方向をチェックするか
    // 0,1 = 上、　0,-1 = 下、　1,1 = 右上 etc...
    let dx = vec![0, 0, 1, 1, 1, -1, -1, -1];
    let dy = vec![1, -1, 1, 0, -1, 1, 0, -1];

    for i in 0..h {
        for j in 0..w {
            if vec[i as usize][j as usize] == '#' {
                continue;
            }

            let mut count = 0;

            // 8方向それぞれチェックする
            for d in 0..8 {
                let di = i + dx[d]; // x軸
                let dj = j + dy[d]; // y軸

                // 範囲外にアクセスしようとした時
                if di < 0 || h <= di {
                    continue;
                }

                // 範囲外にアクセスしようとした時
                if dj < 0 || w <= dj {
                    continue
                }

                if vec[di as usize][dj as usize] == '#' {
                    count += 1;
                }
            }

            vec[i as usize][j as usize] = char::from_digit(count, 10).unwrap();
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, Vec<&'static str>, Vec<Vec<char>>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 5, vec![".....", ".#.#.", "....."], vec![vec!['1', '1', '2', '1', '1'], vec!['1', '#', '2', '#', '1'], vec!['1', '1', '2', '1', '1']]),
            TestCase(3, 5, vec!["#####", "#####", "#####"], vec![vec!['#', '#', '#', '#', '#'], vec!['#', '#', '#', '#', '#'], vec!['#', '#', '#', '#', '#']]),
            TestCase(6, 6, vec!["#####.", "#.#.##", "####.#", ".#..#.", "#.##..", "#.#..."], vec![vec!['#', '#', '#', '#', '#', '3'], vec!['#', '8', '#', '7', '#', '#'], vec!['#', '#', '#', '#', '5', '#'], vec!['4', '#', '6', '5', '#', '2'], vec!['#', '5', '#', '#', '2', '1'], vec!['#', '4', '#', '3', '1', '0']]),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(expected, run(h, w, s));
        }

    }
}
