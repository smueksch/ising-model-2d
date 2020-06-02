use fixedbitset::FixedBitSet;
use std::ops::Index;

use super::spin::Spin;
use super::utils;

/// Struct representing grid dimensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// Struct representing a position within a grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    /// Generate a random position within the provided grid dimensions.
    pub fn random_within_grid(grid_dims: &Dimensions) -> Position {
        Position {
            x: utils::random_u32_up_to(grid_dims.width),
            y: utils::random_u32_up_to(grid_dims.height),
        }
    }
}

/// Shallow wrapper for FixedBitSet.
///
/// Shallow wrapper for FixedBitSet to provide abstraction of the internal
/// data representation. With this struct, bits can simply be addressed
/// using coordinates on a rectangular grid.
pub struct SpinGrid {
    dims: Dimensions,
    spins: FixedBitSet,
}

impl SpinGrid {
    pub fn new(dims: &Dimensions) -> SpinGrid {
        let num_spins = (dims.width * dims.height) as usize;
        let spins = FixedBitSet::with_capacity(num_spins);

        SpinGrid {
            dims: dims.clone(),
            spins,
        }
    }

    /// Return dimensions of the grid.
    pub fn dimensions(&self) -> &Dimensions {
        &self.dims
    }

    /// Return pointer to the start of the spins data.
    pub fn ptr(&self) -> *const u32 {
        self.spins.as_slice().as_ptr()
    }

    /// Flip spin direction at given position.
    ///
    /// # Panics
    ///
    /// Panics if the given position is outside the grid's boundaries.
    pub fn flip(&mut self, pos: Position) {
        if pos.x >= self.dims.width || pos.y >= self.dims.height {
            panic!(format!("{:?} outside of spin grid boundaries.", pos));
        } else {
            let idx = self.calc_index(pos);
            self.spins.toggle(idx);
        }
    }

    /// Sum neighbor spin values for given position.
    ///
    /// Neighbors are the spins directly above or below, left or right of
    /// the given position. Diagonal spins are not considered neighboring.
    ///
    /// For this computation, a spin up is taken to be 1.0 and a spin down
    /// is taken to be -1.0.
    pub fn sum_neighbors(&self, pos: Position) -> f64 {
        let top_neighbor = self.calc_index(Position {
            x: pos.x,
            y: (pos.y - 1) % self.dims.height,
        });
        let bottom_neighbor = self.calc_index(Position {
            x: pos.x,
            y: (pos.y + 1) % self.dims.height,
        });
        let left_neighbor = self.calc_index(Position {
            x: (pos.x - 1) % self.dims.width,
            y: pos.y,
        });
        let right_neighbor = self.calc_index(Position {
            x: (pos.x + 1) % self.dims.width,
            y: pos.y,
        });

        let neighbors = [
            self.spins[top_neighbor],
            self.spins[bottom_neighbor],
            self.spins[left_neighbor],
            self.spins[right_neighbor],
        ];

        neighbors.iter().map(|s| f64::from(Spin::from(s))).sum()
    }

    /// Compute the average spin over the grid.
    ///
    /// For this computation, a spin up is taken to be 1.0 and a spin down
    /// is taken to be -1.0.
    pub fn avg(&self) -> f64 {
        let num_spin_up = self.spins.count_ones(..) as f64;
        let num_spins = self.spins.len() as f64;

        (2.0 * num_spin_up) / num_spins - 1.0
    }

    /// Calculate the index in representation for given position.
    pub(crate) fn calc_index(&self, pos: Position) -> usize {
        (self.dims.width * pos.y + pos.x) as usize
    }
}

impl Index<Position> for SpinGrid {
    type Output = Spin;

    /// Returns a reference to the spin at the given grid position.
    ///
    /// # Panics
    ///
    /// Panics if the position is outside of the grid's boundaries.
    fn index(&self, pos: Position) -> &Self::Output {
        if pos.x >= self.dims.width || pos.y >= self.dims.height {
            panic!(format!("{:?} outside of spin grid boundaries.", pos));
        } else {
            let idx = self.calc_index(pos);
            self.spins[idx].as_ref()
        }
    }
}
