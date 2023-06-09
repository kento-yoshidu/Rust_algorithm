#[allow(dead_code)]
pub fn run(str: &str) -> i32 {
    let mut count = 0;

    for c in str.chars() {
        if c == '#' {
            count += 1;
        }
    }

    count
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
}
