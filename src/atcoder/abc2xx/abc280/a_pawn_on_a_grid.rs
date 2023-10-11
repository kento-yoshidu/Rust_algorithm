// https://atcoder.jp/contests/abc280/tasks/abc280_a

pub fn run(str: &str) -> i32 {
    let mut count = 0;

    for c in str.chars() {
        if c == '#' {
            count += 1;
        }
    }

    count
}

pub fn run2(str: &str) -> usize {
    str.chars().filter(|c| {
        *c == '#'
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str = "#....
                        .....
                        .##..";

        let str2 = "#.#.#
                        ....#
                        ..##.
                        ####.
                        ..#..
                        #####";

        assert_eq!(3, run(str));
        assert_eq!(0, run(".........."));
        assert_eq!(16, run(str2));
    }

    #[test]
    fn test2() {
        let str = "#....
                        .....
                        .##..";

        let str2 = "#.#.#
                        ....#
                        ..##.
                        ####.
                        ..#..
                        #####";

        assert_eq!(3, run2(str));
        assert_eq!(0, run2(".........."));
        assert_eq!(16, run2(str2));
    }
}
