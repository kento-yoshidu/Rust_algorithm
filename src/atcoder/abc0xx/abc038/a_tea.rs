// https://atcoder.jp/contests/abc038/tasks/abc038_a

fn run(s: &str) -> &'static str {
	if s.chars().last().unwrap() == 'T' {
		"YES"
	} else {
		"NO"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(&'static str, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase("ICEDT", "YES"),
			TestCase("MUGICHA", "NO"),
			TestCase("OOLONGT", "YES"),
			TestCase("T", "YES"),
			TestCase("TEA", "NO"),
		];

		for TestCase(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
