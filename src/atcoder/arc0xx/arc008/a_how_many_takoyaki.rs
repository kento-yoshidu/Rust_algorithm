// https://atcoder.jp/contests/arc008/tasks/arc008_1

pub fn run(n: usize) -> usize {
    let a = n*15;
    let b = (n/10)*100 + (n%10)*15;
    let c = ((n/10)+1)*100;

    std::cmp::min(a, std::cmp::min(b, c))
}

