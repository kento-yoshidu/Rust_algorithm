// https://atcoder.jp/contests/abc165/tasks/abc165_b

fn calc(saving: u128, input: u128, year: u128) -> u128 {
    if saving >= input {
        year
    } else {
        calc(saving * 101 / 100, input, year+1)
    }
}

pub fn run(input: f64) -> i32 {
    let mut year = 0;
    let mut saving = 100.0;

    while saving < input {
        saving += (saving / 100.0).floor();
        year += 1;
    }

    year
}

pub fn run2(input: u128) -> u128 {
    calc(100, input, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(103.0));
        assert_eq!(3760, run(1000000000000000000.0));
        assert_eq!(1706, run(1333333333.0));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2(103));
        assert_eq!(3760, run2(1000000000000000000));
        assert_eq!(1706, run2(1333333333));
    }
}
