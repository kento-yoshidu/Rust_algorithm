// https://atcoder.jp/contests/abc163/tasks/abc163_c

fn run(a: isize, b: isize, c: isize, d: isize) -> &'static str {
    let mut t_hp = a;
    let t_at = b;
    let mut a_hp = c;
    let a_at = d;

    while t_hp >= 0 && a_hp >= 0 {
        a_hp = a_hp - t_at;

        if a_hp <= 0 {
            return "Yes";
        }

        t_hp = t_hp - a_at;

        if t_hp <= 0 {
            return "No";
        }

    };

    unreachable!();
}

fn run2(a: isize, b: isize, c: isize, d: isize) -> &'static str {
    let t_hp = a;
    let t_at = b;
    let a_hp = c;
    let a_at = d;

    let takahashi = (t_hp as f64 / a_at as f64).ceil();
    let aoki = (a_hp as f64 / t_at as f64).ceil();

    if takahashi >= aoki {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, &'static str);

    #[test]
    fn abc164_b() {
        let tests = [
            TestCase(10, 9, 10, 10, "No"),
            TestCase(46, 4, 40, 5, "Yes"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
            assert_eq!(run2(a, b, c, d), expected);
        }
    }
}
