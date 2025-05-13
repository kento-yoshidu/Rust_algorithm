// https://atcoder.jp/contests/abc070/tasks/abc070_a

fn run(s: &str) -> &'static str {
	if s.chars().eq(s.chars().rev()) {
		"Yes"
	} else {
		"No"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(&'static str, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase("575", "Yes"),
			TestCase("123", "No"),
			TestCase("812", "No"),
		];

		for TestCase(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
