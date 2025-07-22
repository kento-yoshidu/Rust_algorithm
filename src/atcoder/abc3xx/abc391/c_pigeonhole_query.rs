// https://atcoder.jp/contests/abc391/tasks/abc391_c

pub fn run(n: usize, q: usize, query: Vec<(usize, Option<(usize, usize)>)>) -> Vec<usize> {
    let mut counts = vec![1; n];
    let mut pos: Vec<usize>= (0..n).collect();

    let mut count = 0;

    let mut ans = Vec::new();

    for (n, ph) in query {
        match n {
            1 => {
                let (p, h) = ph.unwrap();

                if counts[pos[p-1]] == 2 {
                    count -= 1;
                }

                counts[pos[p-1]] -= 1;

                pos[p-1] = h-1;
                counts[pos[h-1]] += 1;

                if counts[pos[p-1]] == 2 {
                    count+= 1;
                }
            },
            2 => {
                ans.push(count);
            }
            _ => unreachable!(),
        }
    }

    ans
}
