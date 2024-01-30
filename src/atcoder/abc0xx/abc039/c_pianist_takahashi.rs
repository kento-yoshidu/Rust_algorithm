// https://atcoder.jp/contests/abc039/tasks/abc039_c

pub fn run(s: &str) -> &'static str {
    let str = "WBWBWWBWBWBWWBWBWWBWWBWBWWBWBWBWWBWBWWBW";

    let ans = ["Do", "", "Re", "", "Mi", "Fa", "", "So", "", "La", "", "Si"];

    let p = str.find(s).unwrap();

    ans[p]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Do", run("WBWBWWBWBWBWWBWBWWBW"));
        assert_eq!("Re", run("WBWWBWBWBWWBWBWWBWWB"));
        assert_eq!("Mi", run("WWBWBWBWWBWBWWBWWBWB"));
        assert_eq!("Fa", run("WBWBWBWWBWBWWBWWBWBW"));
        assert_eq!("So", run("WBWBWWBWBWWBWWBWBWWB"));
        assert_eq!("La", run("WBWWBWBWWBWWBWBWWBWB"));
        assert_eq!("Si", run("WWBWBWWBWWBWBWWBWBWB"));
    }
}
