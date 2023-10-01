// https://atcoder.jp/contests/abc092/tasks/abc092_a

fn run(a: usize, b: usize, c: usize, d: usize) -> usize {
	a.min(b) + c.min(d)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(520, run(600, 300, 220, 420));
		assert_eq!(755, run(555, 555, 400, 200));
		assert_eq!(1152, run(549, 817, 715, 603));
	}
}
