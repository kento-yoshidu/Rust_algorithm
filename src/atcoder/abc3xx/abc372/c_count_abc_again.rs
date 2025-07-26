// https://atcoder.jp/contests/abc372/tasks/abc372_c

pub fn run(_n: usize, _q: usize, s: &str, xc: Vec<(usize, char)>) {
    let mut chars: Vec<char> = s.chars().collect();

    let mut count = chars.windows(3)
        .filter(|arr| {
            *arr == ['A', 'B', 'C']
        })
        .count() as isize;

    let mut ans = Vec::new();

    println!("{count}");

    for (x, c) in xc {
        let i = x-1;

        for idx in i-2..=i {
            if chars[idx] == 'A' && chars[idx+1] == 'B' && chars[idx+2] == 'C' {
                count -= 1;
            }
        }

        chars[x-1] = c;

        for idx in i-2..=i {
            if chars[idx] == 'A' && chars[idx+1] == 'B' && chars[idx+2] == 'C' {
                count += 1;
            }
        }

        ans.push(count)
    }

    println!("{:?}", ans);
}