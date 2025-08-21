// https://atcoder.jp/contests/abc226/tasks/abc226_c

use std::collections::{HashMap, VecDeque};

pub fn run(n: usize, tka: Vec<(usize, usize, Option<Vec<usize>>)>) {
    // let mut graph = vec![Vec::new(); n+1];
    // let mut indegree = vec![0; n+1];
    // let mut t = vec![0; n+1];

    // for (i, (t, k, a)) in tka.into_iter().enumerate() {
    //     let idx = i+1;
    //     t[idx] = t;
    //     indegree[idx] = k;
    //     for &d in &a {
    //         graph[d].push(idx);
    //     }
    // }

    // let mut time = vec![0; n+1];
    // let mut q = VecDeque::new();

    // for i in 1..=n {
    //     if indegree[i] == 0 {
    //         time[i] = t[i];
    //         q.push_back(i);
    //     }
    // }

    // while let Some(cur) = q.pop_front() {
    //     for &next in &graph[cur] {
    //         time[next] = time[next].max(time[cur] + t[next]);
    //         indegree[next] -= 1;
    //         if indegree[next] == 0 {
    //             q.push_back(next);
    //         }
    //     }
    // }

    // println!("{}", time[n]);
}
