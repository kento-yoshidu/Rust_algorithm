// https://atcoder.jp/contests/abc350/tasks/abc350_c

use std::mem::swap;

pub fn run(n: usize, a: Vec<usize>) -> Vec<(usize, usize)> {
    let mut a: Vec<usize> = a.into_iter().map(|x| x-1).collect();
    let mut vec = vec![0; n];

    for i in 0..n {
        vec[a[i]] = i;
    }

    let mut ans = Vec::new();

    for i in 0..n {
        if i != a[i] {
            let j = vec[i];

            vec.swap(i, j);
            a.swap(i, j);
            ans.push((i+1, j+1));
        }
        println!("{:?}", vec);
    }

    if ans.is_empty() {
        vec![(0, 0)]
    } else  {
        ans
    }
}
