// https://atcoder.jp/contests/kupc2017/tasks/kupc2017_a

fn calc(count: usize, rest: usize, vec: Vec<usize>) -> isize {
    if vec[count] >= rest {
        count as isize + 1
    } else {
        calc(count+1, rest - vec[count+1], vec)
    }
}

pub fn run(_n: usize, k: usize, a: Vec<usize>) -> isize {
    let sum: usize = a.iter().sum();

    if sum < k {
        return -1
    }

    let mut vec = a.clone();
    vec.sort();
    vec.reverse();

    calc(0, k, vec)
}
