// https://atcoder.jp/contests/abc311/tasks/abc311_d

use std::collections::VecDeque;

pub fn run(n: usize, m: usize, s: Vec<&str>) -> usize {
    let vec: Vec<Vec<char>> = s.into_iter().map(|str| str.chars().collect()).collect();

    let mut temp = 0;

    let mut seen = vec![vec![false; m]; n];
    seen[1][1] = true;

    for i in 0..n {
        for j in 0..m {
            if vec[i][j] == '#' {
                seen[i][j] = true;
            } else {
                temp += 1;
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((1, 1, 'u'));
    queue.push_back((1, 1, 'd'));
    queue.push_back((1, 1, 'l'));
    queue.push_back((1, 1, 'r'));

    while let Some((cur_i, cur_j, dis)) = queue.pop_front() {
        let (mut new_i, mut new_j) = match dis {
            'u' => (cur_i - 1, cur_j),
            'd' => (cur_i + 1, cur_j),
            'l' => (cur_i, cur_j - 1),
            'r' => (cur_i, cur_j + 1),
            _ => unreachable!(),
        };

        if vec[new_i as usize][new_j as usize] == '#' /* || seen[new_i as usize][new_j as usize] */ {
            continue;
        }

        seen[new_i as usize][new_j as usize] = true;

        // 壁にぶつかるまで進む
        loop {
            match dis {
                'u' => new_i -= 1,
                'd' => new_i += 1,
                'l' => new_j -= 1,
                'r' => new_j += 1,
                _ => unreachable!(),
            }

            if vec[new_i as usize][new_j as usize] == '#' {
                match dis {
                    'u' => new_i += 1,
                    'd' => new_i -= 1,
                    'l' => new_j += 1,
                    'r' => new_j -= 1,
                    _ => unreachable!(),
                }

                break;
            }

            seen[new_i as usize][new_j as usize] = true;
        }

        queue.push_back((new_i, new_j, 'u'));
        queue.push_back((new_i, new_j, 'd'));
        queue.push_back((new_i, new_j, 'l'));
        queue.push_back((new_i, new_j, 'r'));
    }

    let mut ans = 0;

    for s in seen.iter() {
        for b in s {
            if !b {
                ans += 1;
            }
        }
    }

    println!("{}", temp - ans);

    ans
}
