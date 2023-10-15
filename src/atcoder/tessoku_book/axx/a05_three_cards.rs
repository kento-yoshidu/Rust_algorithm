// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_e

pub fn run(n: usize, k: usize) -> usize {
    let mut count = 0;

    for x in 1..=n {
        for y in 1..=n {
            if x + y >= k {
                break;
            }

            if k - x - y <= n {
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
        assert_eq!(7, run(3, 6));
        assert_eq!(6498498, run(3000, 4000));
    }
}
