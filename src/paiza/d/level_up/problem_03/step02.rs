// https://paiza.jp/works/mondai/d_rank_level_up_problems/d_rank_level_up_problems__stdin_2

fn run<'a>(s: &'a str, t: &'a str) -> (&'a str, &'a str) {
    (s, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, (&'static str, &'static str));

    #[test]
    fn paiza_d_level_up_03_step02() {
        let tests = [
            TestCase("paiza", "gino", ("paiza", "gino")),
            TestCase("heisei31", "reiwa1", ("heisei31", "reiwa1")),
            TestCase("2012021300", "2020042218", ("2012021300", "2020042218")),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
