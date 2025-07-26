// https://atcoder.jp/contests/abc387/tasks/abc387_d

use std::collections::VecDeque;

fn out_of_bounds(i: isize, j: isize, h: isize, w: isize) -> bool {
    i < 0 || j < 0 || i >= h || j >= w
}

fn run(h: usize, w: usize, s: Vec<&str>) -> isize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut pos_s = (0, 0);
    let mut pos_g = (0, 0);

    for h in 0..h {
        for w in 0..w {
            if vec[h][w] == 'S' {
                pos_s = (h, w);
            }

            if vec[h][w] == 'G' {
                pos_g = (h, w);
            }
        }
    }

    let d = [1, -1];

    // 縦移動始まりと横移動始まりをそれぞれ試す
    (0..2)
        .filter_map(|i| {
            let mut graph = vec![vec![-1; w]; h];
            graph[pos_s.0][pos_s.1] = 0;

            let mut queue = VecDeque::new();
            queue.push_back(pos_s);

            while let Some((cur_i, cur_j)) = queue.pop_front() {
                for j in 0..2 {
                    let (new_i, new_j) = if (i + cur_i + cur_j) % 2 == 0 {
                        (cur_i as isize + d[j], cur_j as isize)
                    } else {
                        (cur_i as isize, cur_j as isize + d[j])
                    };

                    if out_of_bounds(new_i, new_j, h as isize, w as isize) {
                        continue;
                    }

                    let new_i = new_i as usize;
                    let new_j = new_j as usize;

                    if vec[new_i][new_j] == '#' || graph[new_i][new_j] != -1 {
                        continue;
                    }

                    graph[new_i][new_j] = graph[cur_i][cur_j] + 1;
                    queue.push_back((new_i, new_j));
                }
            }

            (graph[pos_g.0][pos_g.1] != -1).then_some(graph[pos_g.0][pos_g.1])
        })
        .min()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, isize);

    #[test]
    fn abc387_d() {
        let tests = [
            TestCase(3, 5, vec![".S#.G", ".....", ".#..."], 7),
            TestCase(3, 5, vec!["..#.G", ".....", "S#..."], -1),
            TestCase(8, 63, vec!["...............................................................","..S...#............................#####..#####..#####..####G..","..#...#................................#..#...#......#..#......","..#####..####...####..####..#..#...#####..#...#..#####..#####..","..#...#..#..#...#..#..#..#..#..#...#......#...#..#..........#..","..#...#..#####..####..####..####...#####..#####..#####..#####..", "................#.....#........#...............................", "................#.....#........#..............................."], 148),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
