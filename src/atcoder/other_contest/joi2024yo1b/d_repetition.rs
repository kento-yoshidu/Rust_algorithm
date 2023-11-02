// https://atcoder.jp/contests/joi2024yo1b/tasks/joi2024_yo1b_d

fn calc(state: usize, n: usize, count: usize) -> usize {
    if n <= state {
        return count
    }

    match state % 3 {
        0 => calc(state+1, n, count+1),
        1 => calc(state*2, n, count+1),
        2 => calc(state*3, n, count+1),
        _ => unreachable!(),
    }
}

pub fn run(x: usize, n: usize) -> usize {
    calc(x, n, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(2, 40));
        assert_eq!(1, run(3, 4));
        assert_eq!(3, run(20, 62));
        assert_eq!(19, run(1, 100000));
    }
}
