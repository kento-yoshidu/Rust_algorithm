// https://atcoder.jp/contests/abc001/tasks/abc001_3

fn run(der: usize, w: usize) -> (&'static str, usize) {
    let w = (w as f64 / 60.0 * 10.0).round()  / 10.0;

    if w <= 0.2 {
        return ("C", 0);
    }

    let der = der as f64 / 10.0;

    let der = match der {
        11.25..33.75 => "NNE",
        33.75..56.25 => "NE",
        56.25..78.75 => "ENE",
        78.75..101.25 => "E",
        101.25..123.75 => "ESE",
        123.75..146.25 => "SE",
        146.25..168.75 => "SSE",
        168.75..191.25 => "S",
        191.25..213.75 => "SSW",
        213.75..236.25 => "SW",
        236.25..258.75 => "WSW",
        258.75..281.25 => "W",
        281.25..303.75 => "WNW",
        303.75..326.25 => "NW",
        326.25..349.75  => "NNW",
        _ => "N",
    };

    let w = match w {
        0.3..=1.5 => 1,
        1.6..=3.3 => 2,
        3.4..=5.4 => 3,
        5.5..=7.9 => 4,
        8.0..=10.7 => 5,
        10.8..=13.8 => 6,
        13.9..=17.1 => 7,
        17.2..=20.7 => 8,
        20.8..=24.4 => 9,
        24.5..=28.4 => 10,
        28.5..=32.6 => 11,
        _ => 12,
    };

    (der, w as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, (&'static str, usize));

    #[test]
    fn abc001_c() {
        let tests = [
            TestCase(2750, 628, ("W", 5)),
            TestCase(161, 8, ("C", 0)),
            TestCase(3263, 15, ("NNW", 1)),
            TestCase(1462, 1959, ("SE", 12)),
            TestCase(1687, 1029, ("SSE", 8)),
            TestCase(2587, 644, ("WSW", 5)),
            TestCase(113, 201, ("NNE", 3)),
            TestCase(2048, 16, ("SSW", 1)),
        ];

        for TestCase(der, w, expected) in tests {
            assert_eq!(run(der, w), expected);
        }
    }
}
