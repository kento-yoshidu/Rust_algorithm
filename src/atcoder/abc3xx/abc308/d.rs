// https://atcoder.jp/contests/abc308/tasks/abc308_d

use std::collections::VecDeque;

fn out_of_bounds(h: usize, w: usize, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || i == h as isize || j == w as isize
}

fn run(h: usize, w: usize, s: Vec<&str>) -> &'static str {
    let str = ['s', 'n', 'u', 'k', 'e'];
    let chars: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut dist = vec![vec![false; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    while let Some(((cur_i, cur_j), count)) = queue.pop_front() {
        if &str[count % 5] != &chars[cur_i][cur_j] || dist[cur_i][cur_j] {
            continue;
        }

        if cur_i == h-1 && cur_j == w-1 {
            return "Yes";
        }

        dist[cur_i][cur_j] = true;

        for i in 0..4 {
            let new_i = cur_i as isize +  dx[i];
            let new_j = cur_j as isize +  dy[i];

            if out_of_bounds(h, w, new_i, new_j) {
                continue;
            }

            queue.push_back(((new_i as usize, new_j as usize), count+1));
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc308_d() {
        let tests = [
            TestCase(2, 3, vec!["sns", "euk"], "Yes"),
            TestCase(2, 2, vec!["ab", "cd"], "No"),
            TestCase(5, 7, vec!["skunsek", "nukesnu", "ukeseku", "nsnnesn", "uekukku"], "Yes"),
        ];

        for TestCase(h, w, s, expected) in tests {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
