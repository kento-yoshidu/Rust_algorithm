// https://atcoder.jp/contests/abc073/tasks/abc073_a

fn run(n: usize) -> &'static str {
	if n >= 90 || n % 10 == 9 {
		"Yes"
	} else {
		"No"
	}
}

fn run2(n: usize) -> &'static str {
	if n.to_string().chars().any(|c| {
		c == '9'
	}) {
		"Yes"
	} else {
		"No"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, &'static str);

	#[test]
	fn test() {
		let tests = [
			TestCase(29, "Yes"),
			TestCase(72, "No"),
			TestCase(91, "Yes")
		];

		for TestCase(n, expected) in tests {
			assert_eq!(run(n), expected);
			assert_eq!(run2(n), expected);
		}
	}
}
