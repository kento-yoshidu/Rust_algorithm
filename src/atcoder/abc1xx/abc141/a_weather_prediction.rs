// https://atcoder.jp/contests/abc141/tasks/abc141_a

fn run(s: &str) -> &'static str {
	match s {
		"Sunny" => "Cloudy",
		"Cloudy" => "Rainy",
		"Rainy" => "Sunny",
		_ => unreachable!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCaes(&'static str, &'static str);

	#[test]
	fn abc141_a() {
		let tests = [
			TestCaes("Sunny", "Cloudy"),
			TestCaes("Rainy", "Sunny"),
			TestCaes("Cloudy", "Rainy"),
		];

		for TestCaes(s, expected) in tests {
			assert_eq!(run(s), expected);
		}
	}
}
