// https://atcoder.jp/contests/abc321/tasks/abc321_b

fn calc(n: usize, x: usize, vec: & Vec<usize>, i: usize) -> bool {
    let mut new_vec = vec.to_vec();

    new_vec.push(i);
    new_vec.sort();

    if new_vec[1..(n-1) as usize].iter().sum::<usize>() >= x {
        true
    } else {
        false
    }
}

fn run(n: usize, x: usize, a: Vec<usize>) -> isize {
    (0..101)
        .find(|num| {
            calc(n, x, &a, *num)
        })
        .map(|n| n as isize)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, isize);

    #[test]
    fn abc321_a() {
        let tests = [
            TestCase(5, 180, vec![40, 60, 80, 50], 70),
            TestCase(3, 100, vec![100, 100], 0),
            TestCase(5, 200, vec![0, 0, 99, 99], -1),
            TestCase(10, 480, vec![59, 98, 88, 54, 70, 24, 8, 94, 46], 45),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
