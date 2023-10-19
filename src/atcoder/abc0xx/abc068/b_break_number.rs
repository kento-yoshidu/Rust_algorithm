// https://atcoder.jp/contests/abc068/tasks/abc068_b

fn calc(n: usize, count: usize) -> usize {
    if n % 2 != 0 {
        return count
    } else {
        calc(n / 2, count+1)
    }
}

pub fn run(n: usize) -> usize {
    (1..=n)
        .map(|i| {
            (i, calc(i, 0))
        })
        .filter(|t| {
            t.1 != 0
        })
        .max_by_key(|t| {
            t.1
        })
        .unwrap_or((1, 0)).0
}

fn main() {
    println!("{}", run(7));
}
