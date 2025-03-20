use std::{collections::VecDeque, fmt::Display};

use rand::{Rng, SeedableRng, rngs::SmallRng};

// $\mathtt{\overline{n_1 n_2 n_3 n_4} = 4228}$
pub const DEFAULT_ROWS: &[usize] = &[4, 3, 5];
pub const VERTEX_COUNT: usize = 12; // $\mathtt{10 + n_3} = 12$
const RANDOM_SEED: u64 = 4228;

#[derive(Clone)]
pub struct Graph(pub Vec<Vec<Option<u32>>>);

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                if let Some(weight) = self.0[i][j] {
                    write!(f, "{:>3} ", weight)?;
                } else {
                    write!(f, "{:>3} ", "∞")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Graph {
    pub fn generate(k: f32) -> Self {
        const ITER_LEN: usize = VERTEX_COUNT * VERTEX_COUNT;
        let mut rng = SmallRng::seed_from_u64(RANDOM_SEED);
        let iter = std::iter::repeat_with(move || rng.random_range(0.0..2.0));

        let mut adj_undir: Vec<Vec<u32>> = iter
            .clone()
            .take(ITER_LEN)
            .map(|i| f32::min(i * k, 1.0) as u32)
            .collect::<Vec<_>>()
            .chunks(VERTEX_COUNT)
            .map(|row| row.to_vec())
            .collect();
        for i in 0..VERTEX_COUNT {
            for j in (i + 1)..VERTEX_COUNT {
                adj_undir[j][i] = adj_undir[i][j];
            }
        }

        let c: Vec<Vec<u32>> = iter
            .take(ITER_LEN)
            .enumerate()
            .map(|(i, w)| {
                (w * 100.0 * adj_undir[i / VERTEX_COUNT][i % VERTEX_COUNT] as f32).ceil() as u32
            })
            .collect::<Vec<_>>()
            .chunks(VERTEX_COUNT)
            .map(|row| row.to_vec())
            .collect();

        let d: Vec<Vec<u32>> = c
            .iter()
            .map(|row| row.iter().map(|i| (*i > 0) as u32).collect())
            .collect();

        let h: Vec<Vec<u32>> = d
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, _)| (d[i][j] != d[j][i]) as u32)
                    .collect()
            })
            .collect();

        let mut weights: Vec<Vec<Option<u32>>> = vec![vec![Some(0); VERTEX_COUNT]; VERTEX_COUNT];
        for i in 0..VERTEX_COUNT {
            for j in (i + 1)..VERTEX_COUNT {
                let weight = c[i][j] * (d[i][j] + h[i][j] * (i < j) as u32);
                weights[i][j] = if weight != 0 { Some(weight) } else { None };
                weights[j][i] = if weight != 0 { Some(weight) } else { None };
            }
        }

        Graph(weights)
    }

    pub fn kruskal_step(edges: &mut VecDeque<(usize, usize, u32)>, step: &mut KruskalStep) -> bool {
        if step.tree.len() == step.uf.rank.len() - 1 {
            println!("MST built.");
            step.current = None;
            return false;
        }

        if let Some(current) = step.current {
            if step.uf.find(current.0) == step.uf.find(current.1) {
                println!("{:?} would create a cycle", current);
            } else {
                println!("{:?} added to MST.", current);
                step.uf.union(current.0, current.1);
                step.tree.push(current);
            }
            step.current = edges.pop_front();
        } else if let Some(next) = edges.pop_front() {
            step.current = Some(next);
        } else {
            return false;
        }
        true
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return false;
        }

        match self.rank[x_root].cmp(&self.rank[y_root]) {
            std::cmp::Ordering::Less => {
                self.parent[x_root] = y_root;
            }
            std::cmp::Ordering::Greater => {
                self.parent[y_root] = x_root;
            }
            std::cmp::Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }

        true
    }
}

pub struct KruskalStep {
    pub current: Option<(usize, usize, u32)>,
    pub tree: Vec<(usize, usize, u32)>,
    uf: UnionFind,
}

impl KruskalStep {
    pub fn new(vertex_count: usize) -> Self {
        KruskalStep {
            current: None,
            tree: Vec::with_capacity(vertex_count - 1),
            uf: UnionFind::new(vertex_count),
        }
    }

    pub fn weight_sum(&self) -> usize {
        self.tree.iter().map(|(_, _, w)| *w as usize).sum()
    }
}
