// https://atcoder.jp/contests/abc456/tasks/abc456_b

fn run(s1: Vec<char>, s2: Vec<char>, s3: Vec<char>) -> f64 {
    let mut ans = 0.0;

    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                let mut vec = vec![s1[i], s2[j], s3[k]];

                vec.sort();

                if vec == vec!['4', '5', '6'] {
                    ans += 1.0;
                }
            }
        }
    }

    ans / 216.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<char>, Vec<char>, Vec<char>, f64);

    #[test]
    fn abc456_b() {
        let tests = [
            TestCase(vec!['1', '2', '3', '4', '5', '6'], vec!['1', '2', '3', '4', '5', '6'], vec!['1', '2', '3', '4', '5', '6'], 0.027777777777777776),
            TestCase(vec!['4', '5', '6', '4', '5', '6'], vec!['4', '4', '5', '5', '6', '6'], vec!['6', '5', '4', '4', '5', '6'], 0.2222222222222222),
        ];

        for TestCase(s1, s2, s3, expected) in tests {
            assert_eq!(run(s1, s2, s3), expected);
        }
    }
}
