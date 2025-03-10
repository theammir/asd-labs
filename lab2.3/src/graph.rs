use std::fmt::Display;

use rand::{Rng, SeedableRng, rngs::SmallRng};

// $\mathtt{\overline{n_1 n_2 n_3 n_4} = 4228}$
pub const ROWS: &[usize] = &[4, 3, 5];
pub const VERTEX_COUNT: usize = 12; // $\mathtt{10 + n_3} = 12$
const RANDOM_SEED: u64 = 4228;

#[derive(Clone)]
pub struct AdjMatrix(pub [[u8; VERTEX_COUNT]; VERTEX_COUNT]);

impl Display for AdjMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..VERTEX_COUNT {
            for j in 0..VERTEX_COUNT {
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

pub fn generate_dir_matrix() -> AdjMatrix {
    let mut rng = SmallRng::seed_from_u64(RANDOM_SEED);
    let iter = std::iter::repeat_with(move || rng.random_range(0.0..2.0));

    // $\mathtt{1 - n_3 * 0.02 - n_4 * 0.005 - 0.25}$
    const K: f32 = 1.0 - 2.0 * 0.02 - 8.0 * 0.005 - 0.25;

    let mut values = iter.take(VERTEX_COUNT * VERTEX_COUNT);
    let matrix = std::array::from_fn(|_| std::array::from_fn(|_| values.next().unwrap() * K));

    AdjMatrix(matrix.map(|row| row.map(|value| f32::min(1.0, value) as u8)))
}

pub fn convert_to_undir(dir_matrix: &AdjMatrix) -> AdjMatrix {
    let mut undir_matrix = dir_matrix.clone();
    for i in 0..VERTEX_COUNT {
        for j in (i + 1)..VERTEX_COUNT {
            undir_matrix.0[j][i] = undir_matrix.0[i][j];
        }
    }
    undir_matrix
}
