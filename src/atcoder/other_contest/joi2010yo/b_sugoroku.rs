// https://atcoder.jp/contests/joi2010yo/tasks/joi2010yo_b

fn func(n: usize, count: usize, pos: isize, a: &Vec<isize>, b: &Vec<isize>) -> usize {
    let tmp = pos + b[count];
    let new_pos = tmp + a[tmp as usize];
    println!("pos={}, b={}, a={}", pos, b[count], a[tmp as usize]);

    println!("{:?}", new_pos);

    if new_pos == n as isize {
        count
    } else {
        func(n, count+1, new_pos, &a, &b)
    }
}

pub fn run(n: usize, m: usize, a: Vec<isize>, b: Vec<isize>) -> usize {
    let mut pos = 0;

    println!("{}", func(n, 0, 1, &a, &b));

    10
}
