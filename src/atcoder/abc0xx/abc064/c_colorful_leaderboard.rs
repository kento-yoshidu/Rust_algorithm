// https://atcoder.jp/contests/abc064/tasks/abc064_c

fn run(n: usize, a: Vec<usize>) -> (usize, usize) {
    let mut vec = vec![0; 8];
    let mut over = 0;

    for num in a.iter() {
        match num {
            1..=399 => vec[0] += 1,
            400..=799 => vec[1] += 1,
            800..=1199 => vec[2] += 1,
            1200..=1599 => vec[3] += 1,
            1600..=1999 => vec[4] += 1,
            2000..=2399 => vec[5] += 1,
            2400..=2799 => vec[6] += 1,
            2800..=3199 => vec[7] += 1,
            3200.. => over += 1,
            _ => unreachable!(),
        }
    }

    let count = vec.iter()
        .filter(|num| **num != 0)
        .count();

    if over == 0 {
        (count, count)
    } else {
        (std::cmp::max(count, 1), count + std::cmp::min(n-count, over))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![2100, 2500, 2700, 2700], (2, 2)),
            TestCase(5, vec![1100, 1900, 2800, 3200, 3200], (3, 5)),
            TestCase(20, vec![800, 810, 820, 830, 840, 850, 860, 870, 880, 890, 900, 910, 920, 930, 940, 950, 960, 970, 980, 990], (1, 1)),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
