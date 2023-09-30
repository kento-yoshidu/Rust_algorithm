// https://atcoder.jp/contests/abc155/tasks/abc155_b

pub fn run(arr: Vec<i32>) -> String {
	let result = arr.iter()
        .filter(|&x| x % 2 == 0)
        .all(|&x| x % 3 == 0 || x % 5 == 0);

	if result {
		String::from("APPROVED")
	} else {
		String::from("DENIED")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("APPROVED"), run(vec![6, 7, 9, 10, 31]));
		assert_eq!(String::from("DENIED"), run(vec![28, 27, 24]));
	}
}
