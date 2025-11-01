// https://atcoder.jp/contests/abc139/tasks/abc139_a

fn run(s: &str, t: &str) -> usize {
	let vec_s: Vec<char> = s.chars().collect();
	let vec_t: Vec<char> = t.chars().collect();

	vec_s.into_iter()
		.zip(vec_t)
		.filter(|v| v.0 == v.1 )
		.count()
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestCase(&'static str, &'static str, usize);

	#[test]
	fn abc139_a() {
		let tests = [
			TestCase("CSS", "CSR", 2),
			TestCase("SSR", "SSR", 3),
			TestCase("RRR", "SSS", 0),
		];

		for TestCase(s, t, expected) in tests {
			assert_eq!(run(s, t), expected);
		}
	}
}
