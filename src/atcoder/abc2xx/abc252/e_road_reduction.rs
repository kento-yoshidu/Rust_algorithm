// https://atcoder.jp/contests/abc252/tasks/abc252_e

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

const INF: usize = std::usize::MAX;

fn dijkstra(n: usize, hash_map: &HashMap<usize, Vec<(usize, usize)>>) -> (Vec<usize>, Vec<Option<usize>>) {
    let mut dist = vec![INF; n+1];
    let mut prev = vec![None; n + 1];
    dist[1] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, 1)));

    while let Some(Reverse((cur_cost, cur_i))) = priority_queue.pop() {
        if cur_cost > dist[cur_i] {
            continue;
        }

        let Some(next) = hash_map.get(&cur_i) else {
            continue;
        };

        for (next_cost , next_i) in next {
            let new_cost = cur_cost + next_cost;

            if new_cost >= dist[*next_i] {
                continue;
            }

            dist[*next_i] = new_cost;
            prev[*next_i] = Some(cur_i);
            priority_queue.push(Reverse((new_cost, *next_i)));
        }
    }

    (dist, prev)
}

pub fn run(n: usize, _m: usize, abc: Vec<(usize, usize, usize)>) {
    let mut hash_map = HashMap::new();

    for (a, b, c) in abc {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push((c, b));
        hash_map.entry(b).or_insert_with(|| Vec::new()).push((c, a));
    }

    let (dist, prev) = dijkstra(n, &hash_map);

    // 最短経路木に含まれる辺を記録
    let mut spt_edges = vec![];
    for v in 2..=n {
        if let Some(p) = prev[v] {
            let cost = dist[v] - dist[p]; // この辺のコスト
            spt_edges.push((p, v, cost));
        }
    }

    // 最短経路木の辺の中で、影響が最も小さい辺を探す
    let mut best_edge = None;
    let mut min_cost = INF;

    for &(a, b, c) in &spt_edges {
        if c < min_cost {
            min_cost = c;
            best_edge = Some((a, b, c));
        }
    }

    if let Some((a, b, c)) = best_edge {
        println!("削除すべき道: ({}, {}, コスト {})", a, b, c);
    } else {
        println!("削除すべき道が見つかりませんでした");
    }
}
