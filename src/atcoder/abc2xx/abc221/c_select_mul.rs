// https://atcoder.jp/contests/abc221/tasks/abc221_c

// bit全探索（無理目）
pub fn run(n: usize) -> usize {
    let ans = 0;

    let str: Vec<char> = n.to_string().chars().collect();

    for bit in 1..=(str.len() << 1) {
        let mut left = Vec::<usize>::new();
        let mut right = Vec::<usize>::new();

        for i in 0..str.len() {
            if bit & (1 << i) != 0 {
                left.push(str[i] as usize -48);
            } else {
                right.push(str[i] as usize -48);
            }
        }
    }

    ans
}
