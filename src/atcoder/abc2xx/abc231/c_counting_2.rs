// https://atcoder.jp/contests/abc231/tasks/abc231_c

fn calc(n: usize, num: usize, vec: &Vec<usize>) -> usize {
    let mut left = 0;
    let mut right = n;

    loop {
        let middle = (left + right) / 2;

        if left == right || num == vec[middle] {
            return n - middle;
        }

        if num <= vec[middle] {
            right = middle
        } else {
            left = middle + 1
        }
    }
}

fn run(n: usize, _q: usize, a: Vec<usize>, x: Vec<usize>) -> Vec<usize> {
    let mut vec = a.clone();

    vec.sort();

    x.into_iter()
        .map(|num| {
            calc(n, num, &vec)
        })
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 1, vec![100, 160, 130], vec![120], vec![2]),
            TestCase(5, 5, vec![1, 2, 3, 4, 5], vec![6, 5, 4, 3, 2], vec![0, 1, 2, 3, 4]),
            TestCase(5, 5, vec![804289384, 846930887, 681692778, 714636916, 957747794], vec![ 424238336, 719885387, 649760493, 596516650, 189641422], vec![5, 3, 5, 5, 5]),
        ];

        for TestCase(n, q, a, x, expected) in tests {
            assert_eq!(run(n, q, a, x), expected);
        }
    }
}
