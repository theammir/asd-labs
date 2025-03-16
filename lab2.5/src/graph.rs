use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use rand::{Rng, SeedableRng, rngs::SmallRng};

// $\mathtt{\overline{n_1 n_2 n_3 n_4} = 4228}$
pub const DEFAULT_ROWS: &[usize] = &[4, 3, 5];
pub const VERTEX_COUNT: usize = 12; // $\mathtt{10 + n_3} = 12$
const RANDOM_SEED: u64 = 4228;

#[derive(Clone)]
pub struct AdjMatrix(pub Vec<Vec<u32>>);

impl Display for AdjMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                if i == j {
                    write!(f, "\x1b[31m{}\x1b[0m ", self.0[i][j])?;
                    continue;
                }
                write!(f, "{} ", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Bfs;
#[derive(Debug)]
pub struct Dfs;

pub trait Search {}
impl Search for Bfs {}
impl Search for Dfs {}

#[derive(Debug)]
pub struct SearchStep<S: Search> {
    pub active: usize,
    pub visited: Vec<(usize, usize)>,
    pub queue: VecDeque<usize>,
    pub tree: Vec<(usize, usize)>,
    marker: PhantomData<S>,
}

impl<S: Search> SearchStep<S> {
    pub fn new(active: usize, size: usize) -> Self {
        let mut visited = Vec::with_capacity(size);
        visited.push((active, active));
        let mut queue = VecDeque::with_capacity(size);
        queue.push_back(active);

        Self {
            active,
            visited,
            queue,
            tree: Vec::with_capacity(size),
            marker: PhantomData,
        }
    }
}

pub trait Queue {
    fn push_queue(&mut self, value: usize);
    fn pop_queue(&mut self) -> Option<usize>;
}

impl Queue for SearchStep<Bfs> {
    fn push_queue(&mut self, value: usize) {
        self.queue.push_back(value);
    }

    fn pop_queue(&mut self) -> Option<usize> {
        self.queue.pop_front()
    }
}

impl Queue for SearchStep<Dfs> {
    fn push_queue(&mut self, value: usize) {
        self.queue.push_back(value);
    }

    fn pop_queue(&mut self) -> Option<usize> {
        self.queue.pop_back()
    }
}

impl AdjMatrix {
    pub fn generate(k: f32) -> Self {
        let mut rng = SmallRng::seed_from_u64(RANDOM_SEED);
        let iter = std::iter::repeat_with(move || rng.random_range(0.0..2.0));

        AdjMatrix(
            iter.take(VERTEX_COUNT * VERTEX_COUNT)
                .map(|i| f32::min(i * k, 1.0) as u32)
                .collect::<Vec<_>>()
                .chunks(VERTEX_COUNT)
                .map(|row| row.to_vec())
                .collect(),
        )
    }

    pub fn search_next<S: Search>(&self, step: &mut SearchStep<S>) -> bool
    where
        SearchStep<S>: Queue,
    {
        if let Some(next) = step.pop_queue() {
            step.active = next;
            step.tree.push(
                *step
                    .visited
                    .iter()
                    .find(|(_, to)| *to == next)
                    .unwrap_or(&(next, next)),
            );
            self.0[next].iter().enumerate().for_each(|(i, edge)| {
                if *edge == 1 && !step.visited.iter().any(|(_, to)| *to == i) {
                    step.visited.push((next, i));
                    step.push_queue(i);
                }
            });
            true
        } else {
            if let Some(unvisited) =
                (0..self.0.len()).find(|vertex| !step.tree.iter().any(|(_, to)| to == vertex))
            {
                step.push_queue(unvisited);
                self.search_next::<S>(step);
                return true;
            }
            false
        }
    }
}

impl<S: Search> From<&SearchStep<S>> for AdjMatrix {
    fn from(value: &SearchStep<S>) -> Self {
        let size = value.tree.len();
        let mut result = vec![vec![0_u32; size]; size];
        for (from, to) in &value.tree {
            if *from != *to {
                result[*from][*to] = 1;
            }
        }
        AdjMatrix(result)
    }
}
