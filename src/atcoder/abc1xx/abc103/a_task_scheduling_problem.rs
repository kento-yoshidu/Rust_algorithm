// https://atcoder.jp/contests/abc103/tasks/abc103_a

pub fn run(v: &mut Vec<isize>) -> isize {
	v.sort();

	v.windows(2).map(|vec| {
		(vec[0] - vec[1]).abs()
	}).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(5, run(&mut vec![1, 6, 3]));
		assert_eq!(6, run(&mut vec![11, 5, 5]));
		assert_eq!(0, run(&mut vec![100, 100, 100]));
	}
}
