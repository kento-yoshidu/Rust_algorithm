// https://atcoder.jp/contests/abc467/tasks/abc467_b

fn run(n: usize, abs: Vec<(usize, usize, &str)>) -> usize {
    abs.into_iter()
        .filter_map(|(a, b, s)| {
            match s {
                "keep" => {
                    Some(b - a)
                },
                _ => None
            }
        })
        .sum()
}
