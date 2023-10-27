// https://atcoder.jp/contests/joi2014yo/tasks/joi2014yo_a

pub fn run(vec: Vec<usize>) -> usize {
    vec.iter()
        .map(|score| {
            if *score < 40 {
                40
            } else {
                *score
            }
        })
        .sum::<usize>() / 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(68, run(vec![10, 65, 100, 30, 95]));
        assert_eq!(64, run(vec![40, 95, 0, 95, 50]));
    }
}
