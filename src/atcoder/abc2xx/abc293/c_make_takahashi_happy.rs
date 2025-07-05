// https://atcoder.jp/contests/abc293/tasks/abc293_c

use std::collections::VecDeque;

fn run(h: usize, w: usize, a: Vec<Vec<usize>>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, vec![a[0][0]]));

    let di = [0, 1];
    let dj = [1, 0];

    let mut ans = 0;

    while let Some((cur_i, cur_j, visited)) = queue.pop_front() {
        for i in 0..2 {
            let new_i = cur_i + di[i];
            let new_j = cur_j + dj[i];

            if new_i == h || new_j == w {
                continue;
            }

            if visited.contains(&a[new_i][new_j]) {
                continue;
            }

            if new_i == h-1 && new_j == w-1 {
                ans += 1;
                continue;
            }

            let mut new_visited = visited.clone();

            new_visited.push(a[new_i][new_j]);

            queue.push_back((new_i, new_j, new_visited));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn abc293_c() {
        let tests = [
            TestCase(3, 3, vec![vec![3, 2, 2], vec![2, 1, 3], vec![1, 5, 4]], 3),
            TestCase(10, 10, vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40], vec![41, 42, 43, 44, 45, 46, 47, 48, 49, 50], vec![51, 52, 53, 54, 55, 56, 57, 58, 59, 60], vec![61, 62, 63, 64, 65, 66, 67, 68, 69, 70], vec![71, 72, 73, 74, 75, 76, 77, 78, 79, 80], vec![81, 82, 83, 84, 85, 86, 87, 88, 89, 90], vec![91, 92, 93, 94, 95, 96, 97, 98, 99, 100]], 48620),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, a), expected);
        }
    }
}
