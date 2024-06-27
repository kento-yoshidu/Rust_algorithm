// https://atcoder.jp/contests/abc093/tasks/abc093_a

fn run(s: &str) -> &'static str {
	let mut chars: Vec<char> = s.chars().collect();

	chars.sort();

	if vec!['a', 'b', 'c'] == chars {
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
			TestCase("bac", "Yes"),
			TestCase("bab", "No"),
			TestCase("abc", "Yes"),
			TestCase("aaa", "No"),
		];

		for TestCase(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
