use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Mul, MulAssign},
    vec,
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
        if f.alternate() {
            let degrees = (0..self.0.len()).map(|i| self.degree(i));
            let is_regular = self.is_graph_regular();
            let isolated = (0..self.0.len()).filter(|i| self.is_isolated(*i));
            let pendant = (0..self.0.len()).filter(|i| self.is_pendant(*i));

            write!(f, "{:<27}", "Vertex degrees:")?;
            degrees.for_each(|deg| write!(f, "{} ", deg).map(|_| ()).unwrap());
            writeln!(f, "\nIs regular: {}", is_regular)?;
            if is_regular {
                writeln!(f, "Regularity degree: {}", self.degree(0))?;
            }
            write!(f, "{:<27}", "Isolated vertices:")?;
            isolated.for_each(|vertex| write!(f, "{} ", vertex + 1).map(|_| ()).unwrap());
            write!(f, "\n{:<27}", "Pendant vertices:")?;
            pendant.for_each(|vertex| write!(f, "{} ", vertex + 1).map(|_| ()).unwrap());
        } else {
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
        }
        Ok(())
    }
}

impl<'a> Mul<&'a AdjMatrix> for &AdjMatrix {
    type Output = AdjMatrix;

    fn mul(self, rhs: &'a AdjMatrix) -> Self::Output {
        let mut result = vec![vec![0_u32; self.0.len()]; self.0.len()];

        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                let mut sum = 0_u32;
                for k in 0..self.0.len() {
                    sum += self.0[i][k] * rhs.0[k][j];
                }
                result[i][j] = sum;
            }
        }

        AdjMatrix(result)
    }
}

impl MulAssign<&AdjMatrix> for AdjMatrix {
    fn mul_assign(&mut self, rhs: &AdjMatrix) {
        *self = &*self * rhs;
    }
}

impl<'a> Add<&'a AdjMatrix> for &AdjMatrix {
    type Output = AdjMatrix;

    fn add(self, rhs: &'a AdjMatrix) -> Self::Output {
        AdjMatrix(
            self.0
                .iter()
                .zip(&rhs.0)
                .map(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).map(|(a, b)| a + b).collect())
                .collect(),
        )
    }
}

impl AddAssign<&AdjMatrix> for AdjMatrix {
    fn add_assign(&mut self, rhs: &AdjMatrix) {
        *self = &*self + rhs;
    }
}

impl<'a> BitOr<&'a AdjMatrix> for &AdjMatrix {
    type Output = AdjMatrix;

    fn bitor(self, rhs: &'a AdjMatrix) -> Self::Output {
        AdjMatrix(
            self.0
                .iter()
                .zip(&rhs.0)
                .map(|(row_a, row_b)| {
                    row_a
                        .iter()
                        .zip(row_b.iter())
                        .map(|(a, b)| (*a != 0 || *b != 0) as u32)
                        .collect()
                })
                .collect(),
        )
    }
}

impl BitOrAssign<&AdjMatrix> for AdjMatrix {
    fn bitor_assign(&mut self, rhs: &AdjMatrix) {
        *self = &*self | rhs;
    }
}

impl<'a> BitAnd<&'a AdjMatrix> for &AdjMatrix {
    type Output = AdjMatrix;

    fn bitand(self, rhs: &'a AdjMatrix) -> Self::Output {
        AdjMatrix(
            self.0
                .iter()
                .zip(&rhs.0)
                .map(|(row_a, row_b)| {
                    row_a
                        .iter()
                        .zip(row_b.iter())
                        .map(|(a, b)| (*a != 0 && *b != 0) as u32)
                        .collect()
                })
                .collect(),
        )
    }
}

impl BitAndAssign<&AdjMatrix> for AdjMatrix {
    fn bitand_assign(&mut self, rhs: &AdjMatrix) {
        *self = &*self & rhs;
    }
}

impl AdjMatrix {
    pub fn identity(size: usize) -> Self {
        AdjMatrix(
            (0..size)
                .map(|i| (0..size).map(|j| if i == j { 1 } else { 0 }).collect())
                .collect(),
        )
    }
    pub fn transpose(&self) -> Self {
        AdjMatrix(
            (0..self.0.len())
                .map(|col| self.0.iter().map(|row| row[col]).collect())
                .collect(),
        )
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

    pub fn undir(&self) -> Self {
        let mut undir_matrix = self.clone();
        for i in 0..self.0.len() {
            for j in (i + 1)..self.0.len() {
                undir_matrix.0[j][i] = undir_matrix.0[i][j];
            }
        }
        undir_matrix
    }

    pub fn degree_out(&self, vertex: usize) -> usize {
        self.0[vertex].iter().filter(|i| **i == 1).count()
    }

    pub fn degree_in(&self, vertex: usize) -> usize {
        self.0
            .iter()
            .map(|row| row[vertex])
            .filter(|i| *i == 1)
            .count()
    }

    pub fn degree(&self, vertex: usize) -> usize {
        self.degree_out(vertex) + self.degree_in(vertex)
    }

    pub fn is_graph_regular(&self) -> bool {
        let mut degrees = (0..self.0.len()).map(|vertex| self.degree(vertex));
        if let Some(first) = degrees.next() {
            degrees.all(|deg| deg == first)
        } else {
            true
        }
    }

    pub fn is_isolated(&self, vertex: usize) -> bool {
        self.degree(vertex) == 0
    }

    pub fn is_pendant(&self, vertex: usize) -> bool {
        self.degree(vertex) == 1
    }

    pub fn all_paths_of_2(&self) -> Vec<[usize; 3]> {
        let squared = self * self;
        let capacity: usize = squared.0.iter().flatten().sum::<u32>() as usize;
        let mut paths = Vec::with_capacity(capacity);

        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                let mut remaining = squared.0[i][j];
                if remaining > 0 {
                    for k in 0..self.0.len() {
                        if self.0[i][k] == 1 && self.0[k][j] == 1 {
                            paths.push((i, k, j).into());
                            remaining -= 1;
                        }
                    }
                }
            }
        }
        paths
    }

    pub fn all_paths_of_3(&self) -> Vec<[usize; 4]> {
        let cubed = &(self * self) * self;
        let capacity = cubed.0.iter().flatten().sum::<u32>() as usize;
        let mut paths = Vec::with_capacity(capacity);

        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                let mut remaining = cubed.0[i][j];
                if remaining > 0 {
                    for k in 0..self.0.len() {
                        for l in 0..self.0.len() {
                            if self.0[i][k] == 1 && self.0[k][l] == 1 && self.0[l][j] == 1 {
                                paths.push((i, k, l, j).into());
                                remaining -= 1;
                            }
                        }
                    }
                }
            }
        }
        paths
    }

    pub fn reachability(&self) -> Self {
        let mut result = Self::identity(self.0.len());
        let mut next_power = self.clone();
        for _ in 1..self.0.len() {
            result |= &next_power;
            next_power *= self;
        }
        if self.0.len() > 1 {
            result |= &next_power;
        }
        result
    }

    pub fn connectivity(&self) -> Self {
        let reach = self.reachability();
        &reach & &reach.transpose()
    }

    pub fn conn_components(&self) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::with_capacity(self.0.len());
        let mut lookup: HashMap<Vec<u32>, usize> = HashMap::with_capacity(self.0.len());
        let conn = self.connectivity();

        for (i, row) in conn.0.iter().enumerate() {
            if let Some(group) = lookup.get(row) {
                result[*group].push(i);
            } else {
                result.push(Vec::with_capacity(self.0.len()));
                let index = result.len() - 1;
                lookup.insert(row.to_vec(), index);
                result[index].push(i);
            }
        }

        result
    }

    pub fn condensed(&self) -> Self {
        fn comp_index(components: &[Vec<usize>], vertex: usize) -> std::option::Option<usize> {
            components.iter().position(|comp| comp.contains(&vertex))
        }
        let components = self.conn_components();
        let mut cond_matrix = vec![vec![0; components.len()]; components.len()];
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                if self.0[i][j] == 1 {
                    let comp_i = comp_index(&components, i).unwrap();
                    let comp_j = comp_index(&components, j).unwrap();
                    if comp_i != comp_j {
                        cond_matrix[comp_i][comp_j] = 1;
                    }
                }
            }
        }

        AdjMatrix(cond_matrix)
    }
}
