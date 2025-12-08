use std::{collections::HashSet, mem};

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];

        Self { parent, size }
    }

    pub fn find(&mut self, i: usize) -> usize {
      if self.parent[i] != i {
        self.parent[i] = self.find(self.parent[i]);
      }

      self.parent[i]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
      let mut ra = self.find(a);
      let mut rb = self.find(b);

      if ra == rb {
        return false;
      }

      if self.size[ra] < self.size[rb] {
        mem::swap(&mut ra, &mut rb);
      }

      self.parent[rb] = ra;
      self.size[ra] += self.size[rb];
      true
    }

    pub fn part_size(&mut self, i: usize) -> usize {
      let root = self.find(i);
      self.size[root]
    }

    pub fn all_sizes(&mut self) -> Vec<usize>{
      let mut seen = HashSet::new();
      let mut res = vec![];

      for i in 0..self.parent.len() {
        let root = self.find(i);
        if seen.insert(root) {
          res.push(self.size[root]);
        }
      }

      res
    }

    pub fn num_parts(&mut self) -> usize {
      let mut roots = HashSet::new();
      for i in 0..self.parent.len() {
        roots.insert(self.find(i));
      }

      roots.len()
    }
}
