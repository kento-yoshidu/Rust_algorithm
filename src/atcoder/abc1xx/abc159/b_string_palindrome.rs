// https://atcoder.jp/contests/abc159/tasks/abc159_b

fn check(s: &str) -> bool {
	s.chars().eq(s.chars().rev())
}

fn run(s: &str) -> &'static str {
	let n = s.len();

	if check(&s) && check(&s[0..(n-1)/2]) && check(&s[((n+3)/2)-1..]) {
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
	fn abc159_b() {
		let tests = [
			TestCase("akasaka", "Yes"),
			TestCase("level", "No"),
			TestCase("atcoder", "No"),
			TestCase("igyojknviataivnkjoygizigyojknviaaaivnkjoygi", "No"),
		];

		for TestCase(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
