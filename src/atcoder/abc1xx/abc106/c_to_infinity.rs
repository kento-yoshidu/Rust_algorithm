// https://atcoder.jp/contests/abc106/tasks/abc106_c

fn run(s: &str, k: usize) -> char {
    for i in 0..k {
        if s.chars().nth(i).unwrap() != '1' {
            return s.chars().nth(i).unwrap()
        }
    }

    '1'
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, char);

    #[test]
    fn abc106_c() {
        let tests = [
            TestCase("1", 1, '1'),
            TestCase("6", 1, '6'),
            TestCase("111", 1, '1'),
            TestCase("111", 2, '1'),
            TestCase("111", 3, '1'),
            TestCase("1121", 1000000000000000000, '2'),
            TestCase("1121", 2, '1'),
            TestCase("1121", 3, '2'),
            TestCase("1121", 4, '2'),
            TestCase("1214", 5, '2'),
            TestCase("3822434535651488671388118263222567477765652247558781154667867787813244635482224781486555517652377271", 150250350450550650, '3'),
            TestCase("111111111111111111116436357842221787756172382222733528141237116166658656353162356126412526223", 20, '1'),
            TestCase("111111111111111111117322118756841484815524346474882856582285635434521523587354424471731255584", 975864753642531420, '7'),
            TestCase("1214", 4, '2'),
            TestCase("3", 157, '3'),
            TestCase("299792458", 9460730472580800, '2'),
        ];

        for TestCase(s, k, expected) in tests {
            assert_eq!(run(s, k), expected);
        }
    }
}
