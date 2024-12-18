// https://atcoder.jp/contests/joi2023yo2/tasks/joi2023_yo2_a

use itertools::Itertools;

fn run(_n: usize, a: Vec<isize>) -> Vec<usize> {
    let vec: Vec<isize> = a.clone().into_iter().sorted().collect();

    let min = vec.iter().min().unwrap();
    let max = vec.iter().max().unwrap();

    a.into_iter()
        .map(|i| {
            std::cmp::max((i - *min).abs(), (*max - i).abs()) as usize
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![13, 15, 20], vec![7, 5, 7]),
            TestCase(2, vec![100, 100], vec![0, 0]),
            TestCase(10, vec![440894064, 101089692, 556439322, 34369336, 98417847, 216265879, 623843484, 554560874, 247445405, 718003331], vec![406524728, 616913639, 522069986, 683633995, 619585484, 501737452, 589474148, 520191538, 470557926, 683633995]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
