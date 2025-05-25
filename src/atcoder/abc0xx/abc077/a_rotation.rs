// https://atcoder.jp/contests/abc077/tasks/abc077_a

fn run(str1: &str, str2: &str) -> &'static str {
	if str1.chars().eq(str2.chars().rev()) {
		"YES"
	} else {
		"NO"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(&'static str, &'static str, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase("pot", "top", "YES"),
			TestCase("tab", "bet", "NO"),
			TestCase("eye", "eel", "NO"),
		];

		for TestCase(s, t, expected) in tests {
			assert_eq!(run(s, t), expected);
		}
	}
}
