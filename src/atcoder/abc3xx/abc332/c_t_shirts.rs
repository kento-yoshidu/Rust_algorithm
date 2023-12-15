// https://atcoder.jp/contests/abc332/tasks/abc332_c

fn run(_n: usize, m: usize, s: &str) -> usize {
    let vec: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

    vec.iter()
        .fold((0, (m, 0)), |(ans, (muji, logo)), num| {
            match num {
                0 => {
                    (ans, (m, ans))
                },
                1 => {
                    if muji > 0 {
                        (ans, (muji-1, logo))
                    } else if logo > 0 {
                        (ans, (muji, logo-1))
                    } else {
                        (ans+1, (muji, logo))
                    }
                },
                2 => {
                    if logo > 0 {
                        (ans, (muji, logo-1))
                    } else {
                        (ans+1, (muji, logo))
                    }
                },
                _ => unreachable!(),
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, 1, "112022"));
        assert_eq!(3, run(3, 1, "222"));
        assert_eq!(0, run(2, 1, "01"));
    }
}
