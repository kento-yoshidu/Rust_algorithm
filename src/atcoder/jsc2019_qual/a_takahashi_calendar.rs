#[allow(dead_code)]
pub fn run(mm: usize, dd: usize) -> usize {
    let mut count = 0;

    for m in 2..=mm {
        for d in 20..=dd {
            // 1の位が0, 1ならスキップ
            if d%10 < 2 {
                continue
            }

            if m == ((d/10) * (d%10)) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(15, 40));
        assert_eq!(5, run(12, 31));
        assert_eq!(0, run(1, 1));
    }
}
