// https://atcoder.jp/contests/abc135/tasks/abc135_a

fn run(a: usize, b: usize) -> String {
	if (a + b) % 2 == 0 {
		((a + b) / 2).to_string()
	} else {
		String::from("IMPOSSIBLE")
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, &'static str);

	#[test]
	fn abc135_a() {
		let tests = [
			TestCase(2, 16, "9"),
			TestCase(0, 3, "IMPOSSIBLE"),
			TestCase(998244353, 99824435, "549034394"),
		];

		for TestCase(a, b, expected) in tests {
			assert_eq!(run(a, b), expected);
		}
	}
}
