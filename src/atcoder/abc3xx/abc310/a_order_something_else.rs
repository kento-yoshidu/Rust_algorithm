// https://atcoder.jp/contests/abc310/tasks/abc310_a

fn run(_x: usize, p: usize, q: usize, vec: Vec<usize>) -> usize {
    let min = vec.iter().min().unwrap();

    if q + min < p {
        q + min
    } else {
        p
    }
}

fn main() {
    println!("{}", run(3, 100, 50, vec![60, 20, 40]));
    //=> 70

    println!("{}", run(3, 100, 50, vec![60000, 20000, 40000]));
    //=> 100
}
