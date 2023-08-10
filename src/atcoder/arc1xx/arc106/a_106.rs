// https://atcoder.jp/contests/arc106/tasks/arc106_a

pub fn run(n: i64) -> (i64, i64) {
    for aa in 1..n {
        let a = 3_i64.pow(aa as u32);

        if a > n {
            break
        }

        for bb in 1..n {
            let b = 5_i64.pow(bb as u32);

            if b > n {
                break
            }

            if a+b > n {
                break
            }

            if a+b == n {
                return (aa, bb)
            }
        }
    }

    (-1, -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((4, 2), run(106));
        assert_eq!((-1, -1), run(1024));
        assert_eq!((21, 1), run(10460353208));
    }
}
