// https://paiza.jp/works/mondai/c_rank_level_up_problems/c_rank_string_step6

fn run(s: &str) -> String {
    let parts: Vec<_> = s.split(':').collect();

    let mut h = parts[0].parse::<usize>().unwrap();
    let mut m = parts[1].parse::<usize>().unwrap();

    if m >= 30 {
        h = (h + 1) % 24;
        m = (m + 30) % 60;
    } else {
        m += 30;
    }

    format!("{:02}:{:02}", h, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn paiza_c_level_up_02_step06() {
        let tests = [
            TestCase("01:02", "01:32"),
            TestCase("12:31", "13:01"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
