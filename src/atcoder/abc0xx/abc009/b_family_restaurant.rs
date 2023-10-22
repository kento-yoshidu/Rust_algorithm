// https://atcoder.jp/contests/abc009/tasks/abc009_2


fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();

    vec.sort();
    vec.reverse();
    vec.dedup();

    *vec.iter().nth(1).unwrap()
}

fn main() {
    println!("{}", run(4, vec![100, 200, 300, 300]));
    println!("{}", run(5, vec![50, 370, 819, 433, 120]));
    println!("{}", run(6, vec![100, 100, 100, 200, 200, 200]));
}
