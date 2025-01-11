// https://atcoder.jp/contests/arc031/tasks/arc031_2

use std::collections::VecDeque;

// 境界チェック
fn check(r: isize, c: isize, h: usize, w: usize) -> bool {
    r < 0 || c < 0 || r == h as isize || c == w as isize
}

fn run(a: Vec<&str>) -> &'static str {
    let vec: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();

    for i in 0..10 {
        for j in 0..10 {
            let mut graph = vec![vec![-1; 10]; 10];
            let mut queue = VecDeque::new();
            queue.push_front((i, j));

            graph[i][j] = 0;

            let dx = [0, -1, 0, 1];
            let dy = [-1, 0, 1, 0];

            while let Some((cur_i, cur_j)) = queue.pop_front() {
                for i in 0..4 {
                    let new_i = cur_i as isize + dx[i];
                    let new_j = cur_j as isize + dy[i];

                    if check(new_i, new_j, 10, 10) {
                        continue;
                    }

                    if vec[new_i as usize][new_j as usize] == 'x' || graph[new_i as usize][new_j as usize] != -1 {
                        continue;
                    }

                    graph[new_i as usize][new_j as usize] = graph[cur_i][cur_j] + 1;
                    queue.push_back((new_i as usize, new_j as usize));
                }
            }

            let mut flag = true;

            for i in 0..10 {
                for j in 0..10 {
                    if vec[i][j] == 'o' && graph[i][j] == -1 {
                        flag = false;
                    }
                }
            }

            if flag {
                return "YES";
            }
        }
    }

    "NO"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec!["xxxxxxxxxx", "xoooooooxx", "xxoooooxxx", "xxxoooxxxx", "xxxxoxxxxx", "xxxxxxxxxx", "xxxxoxxxxx", "xxxoooxxxx", "xxoooooxxx", "xxxxxxxxxx"], "YES"),
            TestCase(vec!["xxxxxxxxxx", "xoooooooxx", "xxoooooxxx", "xxxoooxxxx", "xxxxxxxxxx", "xxxxxxxxxx", "xxxxxxxxxx", "xxxoooxxxx", "xxoooooxxx", "xxxxxxxxxx"], "NO"),
            TestCase(vec!["xxxxoxxxxx", "xxxxoxxxxx", "xxxxoxxxxx", "xxxxoxxxxx", "ooooxooooo", "xxxxoxxxxx", "xxxxoxxxxx", "xxxxoxxxxx", "xxxxoxxxxx", "xxxxoxxxxx"], "YES"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
