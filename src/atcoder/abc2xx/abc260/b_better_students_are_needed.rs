// https://atcoder.jp/contests/abc260/tasks/abc260_b

pub fn run(_n: usize, x: usize, y: usize, z: usize, a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut vec: Vec<(usize, usize, usize)> = a.iter()
        .zip(b.iter())
        .enumerate()
        .map(|(i, v)| (i+1, *v.0, *v.1))
        .collect();

    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec[x..].sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    vec[x+y..].sort_by(|a, b| (b.1+b.2).cmp(&(a.1+a.2)).then(a.0.cmp(&b.0)));

    vec[0..x+y+z].sort_by(|a, b| a.0.cmp(&b.0));

    (0..x+y+z)
        .map(|i| {
            vec[i].0
        })
        .collect()
}

fn main() {
    println!("{:?}", run(15, 4, 3, 2, vec![30, 65, 20, 95, 100, 45, 70, 85, 20, 35, 95, 50, 40, 15, 85], vec![0, 25, 45, 35, 65, 70, 80, 90, 40, 55, 20, 20, 45, 75, 100]));
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 1, 0, 2, vec![80, 60, 80, 60, 70, 70], vec![40, 20, 50, 90, 90, 80], vec![1, 4, 5]),
            TestCase(5, 2, 1, 2, vec![0, 100, 0, 100, 0], vec![0, 0, 100, 100, 0], vec![1, 2, 3, 4, 5]),
            TestCase(15, 4, 3, 2, vec![30, 65, 20, 95, 100, 45, 70, 85, 20, 35, 95, 50, 40, 15, 85], vec![0, 25, 45, 35, 65, 70, 80, 90, 40, 55, 20, 20, 45, 75, 100], vec![2, 4, 5, 6, 7, 8, 11, 14, 15]),
        ];

        for TestCase(n, x, y, z, a, b, expected) in tests {
            assert_eq!(run(n, x, y, z, a, b), expected);
        }
    }
}
