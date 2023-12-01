use std::collections::VecDeque;

#[allow(dead_code, unused)]
fn run_lengths(vec: Vec<&Vec<usize>>) -> VecDeque<(usize, usize)> {
    let mut result: VecDeque<(usize, usize)> = VecDeque::new();

    for (i, v) in vec.iter().enumerate() {
        if i == 0 {
            result.push_back((v[1], v[2]));
            continue
        }

        if result.iter().last().unwrap().0 == v[1] {
            result[i-1].1 += v[2];
        } else {
            result.push_back((v[1], v[2]));
        }
    }

    result
}

#[allow(unused)]
pub fn run(n: usize, q: Vec<Vec<usize>>) -> Vec<usize> {
    let mut ans: Vec<usize> = Vec::new();

    let vec_a: Vec<_> = q.iter().filter(|arr| arr[0] == 1).collect();

    let mut r_l = run_lengths(vec_a);

    println!("{:?}", r_l);

    let mut vec_b: Vec<_> = q.iter().filter(|arr| arr[0] == 2).map(|arr| arr[1]).collect();

    let mut ans = Vec::new();

    for mut t in vec_b {
        let mut state = 0;

        while t > 0 {
            if t == r_l[0].0 {
                ans.push(t);
                r_l.pop_front();
                continue;
            } else if t < r_l[0].0 {
                ans.push(t);
                r_l[0].0 -= t;
                continue;
            } else {
                state += t;
                r_l.pop_front();
            }
        }
    }

    ans
}

fn main() {
    println!("{:?}", run(4, vec![vec![1, 2, 3], vec![2, 2], vec![1, 3, 4], vec![2, 3]]));
//    println!("{}", run(4, vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 3, 10]]));
}
