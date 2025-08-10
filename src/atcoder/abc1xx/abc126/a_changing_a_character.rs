// https://atcoder.jp/contests/abc126/tasks/abc126_a

fn run(_n: usize, k: usize, s: &str) -> String {
	s.chars()
		.enumerate()
		.map(|(index, c)| {
			if index == k-1 {
				c.to_ascii_lowercase()
			} else {
				c
			}
		})
		.collect::<String>()
}


#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(usize, usize, &'static str, &'static str);

	#[test]
	fn abc126_a() {
		let tests = [
			TestCase(3, 1, "ABC", "aBC"),
			TestCase(4, 3, "CABA", "CAbA"),
		];

		for TestCase(n, k, s, expected) in tests {
			assert_eq!(run(n, k, s), expected);
		}
	}
}
