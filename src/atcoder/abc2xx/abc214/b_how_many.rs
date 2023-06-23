// https://atcoder.jp/contests/abc214/tasks/abc214_b

#[allow(dead_code)]
pub fn run(s: i32, t: i32) -> i32{
    let mut count = 0;

    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a*b*c > t {
                    break
                }
                if a+b+c > s {
                    break
                }

                if a*b*c <= t {
                    count += 1;
                }
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
        assert_eq!(1, run(0, 0));
        assert_eq!(4, run(1, 0));
        assert_eq!(10, run(2, 5));
        assert_eq!(213, run(10, 10));
        assert_eq!(2471, run(30, 100));
    }
}
