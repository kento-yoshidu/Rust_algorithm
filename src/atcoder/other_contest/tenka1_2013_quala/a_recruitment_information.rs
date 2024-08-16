// https://atcoder.jp/contests/tenka1-2013-quala/tasks/tenka1_2013_qualA_a

fn calc(n: usize) -> usize {
    if n > 130000000 {
        n
    } else {
        calc(n*2)
    }
}
fn run() -> usize {
    calc(42)
}
