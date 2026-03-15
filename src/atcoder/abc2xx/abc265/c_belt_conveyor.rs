// https://atcoder.jp/contests/abc265/tasks/abc265_c

fn run(h: usize, w: usize, n: Vec<&str>) -> String {
    // 現在の座標
    let mut x = 0;
    let mut y = 0;

    // 一度訪れた座標を記録していく
    let mut seen = vec![vec![false; w]; h];

    while seen[y][x] == false {
        seen[y][x] = true;

        match n[y].chars().nth(x).unwrap() {
            'R' => {
                // 右端に到達していたら中止
                if x == w-1 {
                    return format!("{} {}", y+1, x+1);
                } else {
                    x += 1;
                }
            },
            'D' => {
                // 底に到達していたら中止
                if y == h-1 {
                    return format!("{} {}", y+1, x+1)
                } else {
                    y += 1;
                }
            },
            'U' => {
                // 天井に到達していたら中止
                if y == 0 {
                    return format!("{} {}", y+1, x+1)
                } else {
                    y -= 1;
                }
            },
            _ => {
                // 左端に到達していたら中止
                if x == 0 {
                    return format!("{} {}", y+1, x+1)
                } else {
                    x -= 1;
                }
            },
        };
    }

    return String::from("-1");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc265_c() {
        let tests = [
            TestCase(2, 3, vec!["RDU", "LRU"], "1 3"),
            TestCase(2, 3, vec!["RRD", "ULL"], "-1"),
            TestCase(9, 44, vec!["RRDDDDRRRDDDRRRRRRDDDRDDDDRDDRDDDDDDRRDRRRRR","RRRDLRDRDLLLLRDRRLLLDDRDLLLRDDDLLLDRRLLLLLDD", "DRDLRLDRDLRDRLDRLRDDLDDLRDRLDRLDDRLRRLRRRDRR", "DDLRRDLDDLDDRLDDLDRDDRDDDDRLRRLRDDRRRLDRDRDD", "RDLRRDLRDLLLLRRDLRDRRDRRRDLRDDLLLLDDDLLLLRDR", "RDLLLLLRDLRDRLDDLDDRDRRDRLDRRRLDDDLDDDRDDLDR", "RDLRRDLDDLRDRLRDLDDDLDDRLDRDRDLDRDLDDLRRDLRR", "RDLDRRLDRLLLLDRDRLLLRDDLLLLLRDRLLLRRRRLLLDDR", "RRRRDRDDRRRDDRDDDRRRDRDRDRDRRRRRRDDDRDDDDRRR"], "9 5"),
        ];

        for TestCase(h, w, n, expected) in tests {
            assert_eq!(run(h, w, n), expected);
        }
    }
}
