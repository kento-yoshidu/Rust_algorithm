use std::collections::VecDeque;

fn out_of_bounds(n: isize, m: isize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || i == n || j == m
}

fn run(n: usize, m: usize, a: Vec<Vec<usize>>) -> usize {
    let di = [0, 1, 0, -1];
    let dj = [1, 0, -1, 0];

    let mut visited = vec![vec![false; m]; n];
    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 1 || visited[i][j] {
                continue;
            }

            ans += 1;
            visited[i][j] = true;

            let mut queue = VecDeque::new();
            queue.push_front((i, j));

            while let Some((i, j)) = queue.pop_front() {
                for k in 0..4 {
                    let ni = i as isize + di[k];
                    let nj = j as isize + dj[k];

                    if out_of_bounds(n as isize, m as isize, ni as isize, nj as isize) {
                        continue;
                    }

                    let ni = ni as usize;
                    let nj = nj as usize;

                    if a[ni][nj] == 1 || visited[ni][nj] {
                        continue;
                    }

                    visited[ni][nj] = true;
                    queue.push_back((ni, nj));
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn islands_01() {
        let tests = [
            TestCase(3, 3, vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 1),
            TestCase(3, 3, vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 0),
            TestCase(3, 3, vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]], 2),
            TestCase(3, 3, vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 4),
            TestCase(1, 1, vec![vec![0]], 1),
            TestCase(1, 1, vec![vec![1]], 0),
            TestCase(3, 3, vec![vec![0, 1, 1], vec![0, 1, 1], vec![0, 0, 0]], 1),
            TestCase(2, 4, vec![vec![0, 0, 1, 0], vec![1, 0, 1, 0]], 2),
            TestCase(1, 4, vec![vec![0, 0, 0, 0]], 1),
            TestCase(1, 4, vec![vec![1, 1, 1, 1]], 0),
            TestCase(1, 4, vec![vec![1, 0, 1, 0]], 2),
            TestCase(4, 1, vec![vec![0], vec![0], vec![0], vec![0]], 1),
            TestCase(4, 1, vec![vec![1], vec![1], vec![1], vec![1]], 0),
            TestCase(4, 1, vec![vec![1], vec![0], vec![1], vec![0]], 2),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
