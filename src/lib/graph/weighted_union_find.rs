#[derive(Debug)]
pub struct WeightedUnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    // 親との差分
    diff_weight: Vec<isize>,
}

impl WeightedUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            diff_weight: vec![0; n],
        }
    }

    // rootを返す + 経路圧縮
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            let root = self.find(p);

            // 親の重みを足す
            self.diff_weight[x] += self.diff_weight[p];

            self.parent[x] = root;
        }

        self.parent[x]
    }

    // xのrootからの重み
    pub fn weight(&mut self, x: usize) -> isize {
        self.find(x);
        self.diff_weight[x]
    }

    // weight[y] - weight[x]
    pub fn diff(&mut self, x: usize, y: usize) -> isize {
        self.weight(y) - self.weight(x)
    }

    // 「yはxよりwだけ重い」を追加
    pub fn unite(&mut self, x: usize, y: usize, w: isize) -> bool {
        let mut w = w + self.weight(x) - self.weight(y);

        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            return false;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
            w = -w;
        }

        self.parent[y] = x;
        self.diff_weight[y] = w;
        self.size[x] += self.size[y];

        true
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn is_valid(&mut self, x: usize, y: usize, w: isize) -> bool {
        if self.same(x, y) {
            self.diff(x, y) == w
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let mut uf = WeightedUnionFind::new(5);

        for i in 0..5 {
            assert!(uf.same(i, i));
            assert_eq!(uf.weight(i), 0);
        }
    }

    #[test]
    fn test_simple_weight() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, 3); // 1 = 0 + 3

        assert_eq!(uf.diff(0, 1), 3);
        assert_eq!(uf.diff(1, 0), -3);
    }

    #[test]
    fn test_chain_weights() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, 3); // 1 = 0 + 3
        uf.unite(1, 2, 5); // 2 = 1 + 5

        // 2 = 0 + 8
        assert_eq!(uf.diff(0, 2), 8);
        assert_eq!(uf.diff(2, 0), -8);
    }

    #[test]
    fn test_same_group() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, 3);
        uf.unite(2, 3, 4);

        assert!(uf.same(0, 1));
        assert!(!uf.same(0, 2));
    }

    #[test]
    fn test_merge_groups_with_weight() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, 3); // 1 = 0 + 3
        uf.unite(2, 3, 4); // 3 = 2 + 4

        // merge: 1と2をつなぐ
        uf.unite(1, 2, 10); // 2 = 1 + 10

        // 3 = 0 + 3 + 10 + 4 = 17
        assert_eq!(uf.diff(0, 3), 17);
    }

    #[test]
    fn test_contradiction_like_case() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, 3);

        // すでに同じ集合
        assert!(!uf.unite(0, 1, 3));

        // 差分は変わらない
        assert_eq!(uf.diff(0, 1), 3);
    }

    #[test]
    fn test_negative_weight() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, -2); // 1 = 0 - 2

        assert_eq!(uf.diff(0, 1), -2);
        assert_eq!(uf.diff(1, 0), 2);
    }

    #[test]
    fn test_path_compression_weight() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, 3);
        uf.unite(1, 2, 5);
        uf.unite(2, 3, 7);

        // 圧縮
        let _ = uf.find(3);

        let root0 = uf.find(0);
        let parent3 = uf.parent[3];

        assert_eq!(parent3, root0);

        let d03 = uf.diff(0, 3);
        assert_eq!(d03, 15);
    }

    #[test]
    fn test_complex_case() {
        let mut uf = WeightedUnionFind::new(6);

        uf.unite(0, 1, 1);
        uf.unite(1, 2, 2);
        uf.unite(3, 4, 3);
        uf.unite(4, 5, 4);

        uf.unite(2, 3, 10);

        // 5 = 0 + 1 + 2 + 10 + 3 + 4 = 20
        assert_eq!(uf.diff(0, 5), 20);
    }

    #[test]
    fn test_inconsistency_detection() {
        let mut uf = WeightedUnionFind::new(5);

        uf.unite(0, 1, 3);
        uf.unite(1, 2, 5);

        // 本来 0→2 は 8
        assert_eq!(uf.diff(0, 2), 8);

        // 矛盾する制約
        assert!(uf.same(0, 2));
        assert_ne!(uf.diff(0, 2), 10);
    }
}
