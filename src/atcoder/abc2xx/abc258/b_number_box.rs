// https://atcoder.jp/contests/abc258/tasks/abc258_b

pub fn run(n: usize, a: Vec<&str>) -> usize {
    let dx = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let dy = vec![-1, -1, 0, 1, 1, 1, 0, -1];

    let vec: Vec<Vec<char>> = a.iter().map(|str| str.chars().collect()).collect();

    let mut ans = 0;

    for i in 0..n {
        for j in 0..n {
            for k in 0..8 {
                let mut current = (i as i32, j as i32);
                let mut temp: Vec<char> = Vec::new();

                for _ in 0..n {
                    temp.push(vec[current.1 as usize][current.0 as usize]);

                    current.1 = (current.1 + dy[k] + n as i32) % n as i32;
                    current.0 = (current.0 + dx[k] + n as i32) % n as i32;
                }

                let num = temp.iter().collect::<String>().parse::<usize>().unwrap();

                ans = ans.max(num);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec!["1161", "1119", "7111", "1811"], 9786),
            TestCase(10, vec![ "1111111111", "1111111111", "1111111111", "1111111111", "1111111111", "1111111111", "1111111111", "1111111111", "1111111111", "1111111111"], 1111111111),
            TestCase(4, vec!["3496", "4218", "7642", "8272"], 9872),
            TestCase(4, vec!["8564", "6856", "7847", "8976"], 9885),
            TestCase(4, vec!["5775", "8428", "7494", "7576"], 9855),
            TestCase(9, vec!["277598483", "538545598", "713392932", "327993216", "284968664", "289143525", "241271447", "258866355", "965866512"], 999422852),
            TestCase(10, vec!["1874117766", "6569184659", "4789678843", "6861836693", "7274547719", "9544838442", "3559417488", "5762879579", "9914768852", "9548828585"], 9998418465),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
