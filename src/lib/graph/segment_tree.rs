pub struct SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,

{
    n: usize,
    size: usize,
    data: Vec<T>,
    e: T,
    op: F,
}

impl<T, F> SegTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(n: usize, e: T, op: F) -> Self {
        let mut size = 1;
        while size < n {
            size <<= 1;
        }
        Self {
            n,
            size,
            data: vec![e; 2 * size],
            e,
            op,
        }
    }

    pub fn from_slice(v: &[T], e: T, op: F) -> Self {
        let n = v.len();
        let mut st = Self::new(n, e, op);

        for i in 0..n {
            st.data[st.size + i] = v[i];
        }

        for i in (1..st.size).rev() {
            st.data[i] = (st.op)(st.data[2 * i], st.data[2 * i + 1]);
        }

        st
    }

    pub fn set(&mut self, mut i: usize, x: T) {
        debug_assert!(i < self.n, "index out of bounds: {} >= {}", i, self.n);
        i += self.size;
        self.data[i] = x;

        while i > 1 {
            i >>= 1;
            self.data[i] = (self.op)(self.data[2 * i], self.data[2 * i + 1]);
        }
    }

    pub fn get(&self, i: usize) -> T {
        debug_assert!(i < self.n, "index out of bounds: {} >= {}", i, self.n);
        self.data[self.size + i]
    }

    // [l, r)
    pub fn prod(&self, mut l: usize, mut r: usize) -> T {
        debug_assert!(l <= r, "invalid range: l={} > r={}", l, r);
        debug_assert!(r <= self.n, "index out of bounds: r={} > n={}", r, self.n);
        let mut sml = self.e;
        let mut smr = self.e;

        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(sml, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(self.data[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(sml, smr)
    }

    pub fn all_prod(&self) -> T {
        self.data[1]
    }
}

impl<T, F> std::fmt::Debug for SegTree<T, F>
where
    T: Copy + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegTree")
            .field("n", &self.n)
            .field("size", &self.size)
            .field("data", &self.data)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum_tree(v: &[i64]) -> SegTree<i64, impl Fn(i64, i64) -> i64> {
        SegTree::from_slice(v, 0, |a, b| a + b)
    }

    fn max_tree(v: &[i64]) -> SegTree<i64, impl Fn(i64, i64) -> i64> {
        SegTree::from_slice(v, i64::MIN, |a, b| a.max(b))
    }

    // --- new / from_slice ---

    #[test]
    fn new_all_identity() {
        let st = SegTree::new(5, 0i64, |a, b| a + b);
        for i in 0..5 {
            assert_eq!(st.get(i), 0);
        }
        assert_eq!(st.all_prod(), 0);
    }

    #[test]
    fn from_slice_get() {
        let v = [1i64, 2, 3, 4, 5];
        let st = sum_tree(&v);
        for (i, &x) in v.iter().enumerate() {
            assert_eq!(st.get(i), x);
        }
    }

    // size が 2 の冪でない場合の境界
    #[test]
    fn non_power_of_two_size() {
        let v = [10i64, 20, 30];
        let st = sum_tree(&v);
        assert_eq!(st.all_prod(), 60);
        assert_eq!(st.prod(0, 3), 60);
    }

    // --- prod (区間和) ---

    #[test]
    fn prod_sum_full_range() {
        let st = sum_tree(&[1, 2, 3, 4, 5]);
        assert_eq!(st.prod(0, 5), 15);
    }

    #[test]
    fn prod_sum_partial() {
        let st = sum_tree(&[1, 2, 3, 4, 5]);
        assert_eq!(st.prod(1, 4), 9); // 2+3+4
    }

    #[test]
    fn prod_single_element() {
        let st = sum_tree(&[1, 2, 3, 4, 5]);
        assert_eq!(st.prod(2, 3), 3);
    }

    #[test]
    fn prod_empty_range() {
        let st = sum_tree(&[1, 2, 3]);
        assert_eq!(st.prod(1, 1), 0); // 単位元
    }

    // --- prod (区間最大値) ---

    #[test]
    fn prod_max_full_range() {
        let st = max_tree(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(st.prod(0, 8), 9);
    }

    #[test]
    fn prod_max_partial() {
        let st = max_tree(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(st.prod(2, 5), 5); // max(4,1,5)
    }

    // --- all_prod ---

    #[test]
    fn all_prod_sum() {
        let st = sum_tree(&[1, 2, 3, 4, 5]);
        assert_eq!(st.all_prod(), 15);
    }

    #[test]
    fn all_prod_max() {
        let st = max_tree(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(st.all_prod(), 9);
    }

    // --- set ---

    #[test]
    fn set_updates_get() {
        let mut st = sum_tree(&[1, 2, 3, 4, 5]);
        st.set(2, 10);
        assert_eq!(st.get(2), 10);
    }

    #[test]
    fn set_propagates_to_prod() {
        let mut st = sum_tree(&[1, 2, 3, 4, 5]);
        st.set(2, 10); // 1+2+10+4+5 = 22
        assert_eq!(st.all_prod(), 22);
        assert_eq!(st.prod(1, 4), 16); // 2+10+4
    }

    #[test]
    fn set_max_propagates() {
        let mut st = max_tree(&[3, 1, 4, 1, 5, 9, 2, 6]);
        st.set(5, 0); // 9 → 0
        assert_eq!(st.all_prod(), 6);
    }

    // --- tessoku-book A58 サンプル ---

    #[test]
    fn tessoku_a58_sample() {
        // 配列長 8、全初期値 0、区間最大値クエリ
        let mut st = SegTree::new(8, 0i64, |a, b| a.max(b));

        st.set(2, 16); // A[3] = 16 (1-indexed) → index 2
        assert_eq!(st.prod(3, 6), 0); // [4,7) 1-indexed → [3,6) 0-indexed

        st.set(4, 13); // A[5] = 13 (1-indexed) → index 4
        assert_eq!(st.prod(3, 6), 13);
    }
}