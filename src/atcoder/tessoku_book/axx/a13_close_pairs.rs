pub fn run(n: usize, k: usize, a: Vec<usize>) -> usize {
    println!("a={:?}", a);

    let mut r = vec![0; n];

    for i in 0..n {
        if i == 0 {
            r[i] = 1;
        } else {
            r[i] = r[i-1];
        }

        println!("r={:?}", r);

        while r[i] < n-1 && a[r[i] + 1] - a[i] <= k {
            println!("差={}", a[r[i]+1] - a[i]);
            r[i] += 1;
        }
    }

    10
}
