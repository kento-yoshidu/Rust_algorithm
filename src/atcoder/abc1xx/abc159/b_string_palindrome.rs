// https://atcoder.jp/contests/abc159/tasks/abc159_b

fn check(s: &str) -> bool {
	println!("{}", s);

	s.chars().eq(s.chars().rev())
}

fn run(s: &str) -> String {
	let n = s.len();

	if check(&s) && check(&s[0..(n-1)/2]) && check(&s[((n+3)/2)-1..]) {
		String::from("Yes")
	} else {
		String::from("No")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Yes"), run("akasaka"));
		assert_eq!(String::from("No"), run("level"));
		assert_eq!(String::from("No"), run("atcoder"));
		assert_eq!(String::from("No"), run("igyojknviataivnkjoygizigyojknviaaaivnkjoygi"));
	}
}