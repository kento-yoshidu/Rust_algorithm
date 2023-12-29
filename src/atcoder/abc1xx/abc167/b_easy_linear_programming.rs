// https://atcoder.jp/contests/abc167/tasks/abc167_b

pub fn run(a: isize, b: isize, _c: isize, k: isize) -> isize {
    if a >= k {
        k
    } else if a + b >= k {
        a
    } else {
        a - (k - (a + b))
    }
}
