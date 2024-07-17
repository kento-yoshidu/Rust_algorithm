// https://atcoder.jp/contests/cf16-final/tasks/codefestival_2016_final_a

pub fn run(_h: usize, _w: usize, s: Vec<Vec<&str>>) -> (char, usize) {
    for (i, vec) in s.into_iter().enumerate() {
        for (j, str) in vec.iter().enumerate() {
            if *str == "snuke" {
                let row = (b'A' + (j) as u8) as char;
                println!("{}", row);

                return (row, i+1);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<&'static str>>, (char, usize));

    #[test]
    fn test() {
        let test = [
            TestCase(15, 10, vec![
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snuke", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
                vec!["snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake", "snake"],
            ], ('H', 6)),
            TestCase(1, 1, vec![vec!["snuke"]], ('A', 1)),
        ];

        for TestCase(h, w, s, expected) in test {
            assert_eq!(run(h, w, s), expected);
        }
    }
}
