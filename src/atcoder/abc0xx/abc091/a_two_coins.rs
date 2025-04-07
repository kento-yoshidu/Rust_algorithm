// https://atcoder.jp/contests/abc091/tasks/abc091_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
	if a + b >= c {
		"Yes"
	} else {
		"No"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, usize, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase(50, 100, 120, "Yes"),
			TestCase(500, 100, 1000, "No"),
			TestCase(19, 123, 143, "No"),
			TestCase(19, 123, 142, "Yes"),
		];

		for TestCase(a, b, c, expected) in tests {
			assert_eq!(run(a, b, c), expected);
		}
	}
}
