// https://atcoder.jp/contests/abc088/tasks/abc088_d

use std::collections::VecDeque;

// 境界チェック
fn check(r: isize, c: isize, h: usize, w: usize) -> bool {
    r < 0 || c < 0 || r >= h as isize || c >= w as isize
}

fn run(h: usize, w: usize, s: Vec<&str>) -> isize {
    let vec: Vec<Vec<char>> = s.iter().map(|str| str.chars().collect()).collect();

    let mut graph: Vec<Vec<isize>> = vec![vec![-1; w]; h];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    graph[0][0] = 0;
    queue.push_back((0, 0));

    let dx = [0, -1, 0, 1];
    let dy = [-1, 0, 1, 0];

    while let Some((cur_i, cur_j)) =  queue.pop_front() {
        for i in 0..4 {
            let new_i = cur_i as isize + dx[i];
            let new_j = cur_j as isize + dy[i];

            if check(new_i, new_j, h, w) {
                continue;
            }

            let (new_i, new_j) = (new_i as usize, new_j as usize);

            if vec[new_i][new_j] == '#' || graph[new_i][new_j] != -1 {
                continue;
            }

            graph[new_i][new_j] = graph[cur_i][cur_j] + 1;
            queue.push_back((new_i, new_j));
        }
    }

    // 右下にたどり着けない場合
    if graph[h-1][w-1] == -1 {
        return -1;
    }

    // #の数
    let count: usize = s.into_iter().map(|str| str.chars().filter(|c| *c == '#').count()).sum();

    // 全体のマス数 - 既存の#の数 - 最短経路のマス数
    (h * w - count - 1) as isize - graph[h-1][w-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec!["..#", "#..", "..."], 2),
            TestCase(10, 37, vec![".....................................", "...#...####...####..###...###...###..", "..#.#..#...#.##....#...#.#...#.#...#.", "..#.#..#...#.#.....#...#.#...#.#...#.", ".#...#.#..##.#.....#...#.#.###.#.###.", ".#####.####..#.....#...#..##....##...", ".#...#.#...#.#.....#...#.#...#.#...#.", ".#...#.#...#.##....#...#.#...#.#...#.", ".#...#.####...####..###...###...###..", "....................................."], 209),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
