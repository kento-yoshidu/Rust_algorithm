#[allow(dead_code)]
pub fn run(h: usize, w: usize, n: Vec<&str>) -> String {
    // 現在の座標
    let mut current = (0, 0);

    // 一度訪れた座標を記録していく
    let mut seen = Vec::new();

    loop {
        match n[current.0].chars().nth(current.1).unwrap() {
            'R' => {
                // 右端に到達していたら中止
                if current.1 == w-1 {
                    return format!("{} {}", current.0+1, current.1+1);
                } else {
                    seen.push(current);
                    current.1 += 1;
                }
            },
            'D' => {
                // 底に到達していたら中止
                if current.0 == h-1 {
                    return format!("{} {}", current.0+1, current.1+1)
                } else {
                    current.0 += 1;
                }
            },
            'U' => {
                // 天井に到達していたら中止
                if current.0 == 0 {
                    return format!("{} {}", current.0+1, current.1+1)
                } else {
                    current.0 -= 1;
                }
            },
            _ => {
                // 左端に到達していたら中止
                if current.1 == 0 {
                    return format!("{} {}", current.0+1, current.1+1)
                } else {
                    current.1 -= 1;
                }
            },
        };

        // 現在の座標が訪問済みの座標にあったらループするので終了
        if seen.iter().any(|&c| c == current) {
            return String::from("-1")
        }
    }
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
