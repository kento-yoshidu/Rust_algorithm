// https://atcoder.jp/contests/abc079/tasks/abc079_a

fn run(s: &str) -> &'static str {
	let chars: Vec<char> = s.chars().collect();

	if 	chars[0] == chars[1] && chars[1] == chars[2] ||
		chars[1] == chars[2] && chars[2] == chars[3] {
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
			TestCase("1118", "Yes"),
			TestCase("1181", "No"),
			TestCase("7777", "Yes"),
			TestCase("1234", "No"),
			TestCase("1122", "No"),
		];

		for TestCase(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
