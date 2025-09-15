// https://atcoder.jp/contests/abc138/tasks/abc138_a

fn run(a: usize, s: &str) -> String {
	if a >= 3200 {
		s.to_string()
	} else {
		String::from("red")
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, &'static str, &'static str);

	#[test]
	fn abc138_a() {
		let tests = [
			TestCase(3200, "pink", "pink"),
			TestCase(3199, "pink", "red"),
			TestCase(4049, "red", "red"),
		];

		for TestCase(a, s, expected) in tests {
			assert_eq!(run(a, s), expected);
		}
	}
}
