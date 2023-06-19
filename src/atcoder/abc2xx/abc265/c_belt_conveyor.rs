#[allow(dead_code)]
pub fn run(h: usize, w: usize, n: Vec<&str>) -> String {
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

    #[test]
    fn test() {
        assert_eq!(String::from("1 3"), run(2, 3, vec!["RDU", "LRU"]));
        assert_eq!(String::from("-1"), run(2, 3, vec!["RRD", "ULL"]));
        assert_eq!(String::from("9 5"), run(9, 44, vec!["RRDDDDRRRDDDRRRRRRDDDRDDDDRDDRDDDDDDRRDRRRRR",
                                        "RRRDLRDRDLLLLRDRRLLLDDRDLLLRDDDLLLDRRLLLLLDD",
                                        "DRDLRLDRDLRDRLDRLRDDLDDLRDRLDRLDDRLRRLRRRDRR",
                                        "DDLRRDLDDLDDRLDDLDRDDRDDDDRLRRLRDDRRRLDRDRDD",
                                        "RDLRRDLRDLLLLRRDLRDRRDRRRDLRDDLLLLDDDLLLLRDR",
                                        "RDLLLLLRDLRDRLDDLDDRDRRDRLDRRRLDDDLDDDRDDLDR",
                                        "RDLRRDLDDLRDRLRDLDDDLDDRLDRDRDLDRDLDDLRRDLRR",
                                        "RDLDRRLDRLLLLDRDRLLLRDDLLLLLRDRLLLRRRRLLLDDR",
                                        "RRRRDRDDRRRDDRDDDRRRDRDRDRDRRRRRRDDDRDDDDRRR"]));
    }
}
