// https://atcoder.jp/contests/abc082/tasks/abc082_a

fn run(a: usize, b: usize) -> usize {
	((a as f64 + b as f64) / 2.0).ceil() as usize
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(2, run(1, 3));
		assert_eq!(6, run(7, 4));
		assert_eq!(5, run(5, 5));
	}
}
