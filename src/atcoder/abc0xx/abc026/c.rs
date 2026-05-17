// https://atcoder.jp/contests/abc026/tasks/abc026_c

pub fn run(n: usize, b: Vec<usize>) -> usize {
    let mut list = vec![vec![]; n];

    for (i, b) in b.iter().enumerate() {
        list[*b].push(i+1);
    }

    let mut arr = vec![0; n+1];

    for i in (1..n).rev() {
        if list[i].len() == 0 {
            arr[i] = 1;

            continue;
        }

        for (j, l) in list.iter().enumerate() {
            if l.contains(&i) {
            }
        }

    }

    0
}
