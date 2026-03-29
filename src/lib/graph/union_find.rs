#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);

            self.parent[x] = root;
        }

        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            return false;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent[y] = x;
        self.size[x] += self.size[y];

        true
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);

        self.size[root]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let mut uf = UnionFind::new(5);
        for i in 0..5 {
            assert!(uf.same(i, i));
            assert_eq!(uf.size(i), 1);
        }
    }

    #[test]
    fn test_simple_union() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);

        assert!(uf.same(0, 1));
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(1), 2);
    }

    #[test]
    fn test_multiple_unions() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        uf.unite(1, 2);

        assert!(uf.same(0, 2));
        assert_eq!(uf.size(0), 3);
    }

    #[test]
    fn test_disjoint_sets() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        uf.unite(2, 3);

        assert!(!uf.same(0, 2));
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(2), 2);
    }

    #[test]
    fn test_union_same_element() {
        let mut uf = UnionFind::new(5);
        assert!(!uf.unite(1, 1)); // すでに同じ
        assert_eq!(uf.size(1), 1);
    }

    #[test]
    fn test_union_chain() {
        let mut uf = UnionFind::new(6);
        uf.unite(0, 1);
        uf.unite(1, 2);
        uf.unite(2, 3);
        uf.unite(3, 4);

        assert!(uf.same(0, 4));
        assert_eq!(uf.size(0), 5);
    }

    #[test]
    fn test_union_by_size() {
        let mut uf = UnionFind::new(6);
        uf.unite(0, 1); // size 2
        uf.unite(2, 3); // size 2
        uf.unite(0, 2); // merge

        let root = uf.find(0);
        assert_eq!(uf.size(root), 4);
    }

    #[test]
    fn test_path_compression_effect() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        uf.unite(1, 2);
        uf.unite(2, 3);

        // findで圧縮
        let root = uf.find(3);

        // 直接rootに繋がっていることを確認
        assert_eq!(uf.parent[3], root);
    }
}
