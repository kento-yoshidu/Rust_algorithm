// https://atcoder.jp/contests/abc077/tasks/abc077_a

fn run(str1: &str, str2: &str) -> String {
	if str1.chars().eq(str2.chars().rev()) {
		String::from("YES")
	} else {
		String::from("NO")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("YES"), run("pot", "top"));
		assert_eq!(String::from("NO"), run("tab", "bet"));
		assert_eq!(String::from("NO"), run("eye", "eel"));
	}
}
