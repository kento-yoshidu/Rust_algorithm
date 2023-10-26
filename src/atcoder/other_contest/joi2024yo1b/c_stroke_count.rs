// https://atcoder.jp/contests/joi2024yo1b/tasks/joi2024_yo1b_c

fn run(_n: usize, s: String) -> usize {
    s.chars()
        .map(|c| {
            match c {
                'j' | 'i' => 2,
                'o' => 1,
                _ => unreachable!(),
            }
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, run(6, String::from("jjooii")));
        assert_eq!(2, run(1, String::from("i")));
        assert_eq!(21, run(13, String::from("joiojiioijoio")));
    }
}
