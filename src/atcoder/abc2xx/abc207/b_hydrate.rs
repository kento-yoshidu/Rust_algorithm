// https://atcoder.jp/contests/abc207/tasks/abc207_b

fn calc(count: isize, state: (usize, usize), d: usize) -> isize {

}

pub fn run(a: usize, b: usize, c: usize, d: usize) -> isize {
    if b < c {
        return -1
    } else if a * d < c {
        return 0
    } else {
        (0, (b, c), d)
    }
}
