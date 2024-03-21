// https://atcoder.jp/contests/abc275/tasks/abc275_d

fn check(i: usize, j: usize, vec: &Vec<Vec<char>>) -> bool {
    for ii in 0..2 {
        for jj in 0..2 {
            if vec[i+ii][j+jj] != '#' {
                return false
            }
        }
    }

    // 右下の黒をチェック
    for ii in 6..8 {
        for jj in 6..8 {
            if vec[i+ii][j+jj] != '#' {
                return false
            }
        }
    }

    // 左上の白をチェック
    for ii in 0..3 {
        if vec[i+ii][j+3] != '.' {
            return false;
        }
    }
    for jj in 0..3 {
        if vec[i+3][j+jj] != '.' {
            return false;
        }
    }

    //右下の白をチェック
    for ii in 5..8 {
        if vec[i+ii][j+5] != '.' {
            return false;
        }
    }
    for jj in 5..8 {
        if vec[i+5][j+jj] != '.' {
            return false;
        }
    }
    true
}

pub fn run(n: usize, m: usize, s: Vec<&str>) -> Vec<(usize, usize)> {
    let vec: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    let mut ans = Vec::new();

    for i in 0..n-8 {
        for j in 0..m-8 {
            if check(i, j, &vec) {
                ans.push((i+1, j+1));
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<(usize, usize)>);

    #[test]
    fn test() {
        let tests = [
            TestCase(19, 18, vec!["###......###......", "###......###......", "###..#...###..#...", "..............#...", "..................", "..................", "......###......###", "......###......###", "......###......###", ".###..............", ".###......##......", ".###..............", "............###...", "...##.......###...", "...##.......###...", ".......###........", ".......###........", ".......###........", "........#........."], vec![(1, 1), (1, 10), (7, 7), (10, 2)]),
            TestCase(9, 21, vec!["###.#...........#.###", "###.#...........#.###", "###.#...........#.###", "....#...........#....", "#########...#########", "....#...........#....", "....#.###...###.#....", "....#.###...###.#....", "....#.###...###.#...."], vec![(1, 1)]),
            TestCase(18, 18, vec![ "######............", "######............", "######............", "######............", "######............", "######............", "..................", "..................", "..................", "..................", "..................", "..................", "............######", "............######", "............######", "............######", "............######", "............######"], vec![]),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, s), expected);
        }
    }
}

/*
// use proconio::input;
use std::collections::HashMap;

fn calc(n: usize, h: &mut HashMap<usize, usize>) -> usize {
    if n == 0 {
        return 1;
    }

    if let Some(x) = h.get(&n) {
        return *x;
    }

    let num = calc(n/2, h) + calc(n/3, h);
    h.entry(n).or_insert(num);

    return num;
}

fn main() {
    input! {
        n: usize,
    }

    let mut hash_map: HashMap<usize, usize> = HashMap::new();
    println!("{}", calc(n, &mut hash_map));
}
*/
