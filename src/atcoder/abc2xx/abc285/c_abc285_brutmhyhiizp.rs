// https://atcoder.jp/contests/abc285/tasks/abc285_c

pub fn run(s: &str) -> usize {
    s.chars()
        .rev()
        .enumerate()
        .fold(0, |state, (i, c)| {
            state + (c as u8 - 64) as usize * 26_usize.pow(i as u32)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(28, run("AB"));
        assert_eq!(3, run("C"));
        assert_eq!(10000000000000000, run("BRUTMHYHIIZP"));
    }
}
