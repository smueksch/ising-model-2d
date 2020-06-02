use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;

mod spin;
mod spin_grid;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
 * Constants
 */

const SPIN_UP: bool = true;
const SPIN_DOWN: bool = false;

pub fn to_spin(spin_repr: bool) -> i32 {
    if spin_repr == SPIN_UP {
        1
    } else {
        -1
    }
}

const SPIN_DOWN_PROBABILITY: f64 = 0.3;

/*
 * Simulation
 */

#[wasm_bindgen]
pub struct Simulation {
    width: u32,
    height: u32,
    coupling: f64,
    magnetic_field: f64,
    spins: FixedBitSet,
}

impl Simulation {
    /// Calculate index from row and column of a spin.
    pub fn index(&self, row: u32, col: u32) -> Result<usize, &str> {
        Ok((row * self.width + col) as usize)
    }

    /// Get spin at given row and column.
    pub fn spin(&self, row: u32, col: u32) -> i32 {
        let index = self.index(row, col).unwrap();
        let spin_repr = self.spins[index];

        to_spin(spin_repr)
    }

    /// Flip spin at row and column position.
    pub fn flip_spin(&mut self, row: u32, column: u32) {
        let index = self.index(row, column).unwrap();
        self.spins.toggle(index);
    }

    /// Return random row and column position in the simulation.
    pub fn random_position(&self) -> (u32, u32) {
        let height = self.height as f64;
        let width = self.width as f64;

        let rand_row = (Math::random() * height) as u32;
        let rand_col = (Math::random() * width) as u32;

        (rand_row, rand_col)
    }

    pub fn calc_neighbor_sum(&self, row: u32, column: u32) -> i32 {
        let top_row = (row - 1) % self.height;
        let bottom_row = (row + 1) % self.height;

        let left_column = (column - 1) % self.width;
        let right_column = (column + 1) % self.width;

        let sum = self.spin(top_row, column)
            + self.spin(bottom_row, column)
            + self.spin(row, left_column)
            + self.spin(row, right_column);

        sum
    }

    /// Return the delta energy.
    pub fn calc_delta_energy(&self, row: u32, column: u32, neighbor_sum: i32) -> f64 {
        let neighbor_sum = neighbor_sum as f64;
        let spin = self.spin(row, column) as f64;

        2.0 * spin * (self.coupling * neighbor_sum + self.magnetic_field)
    }

    /// Update a single spin at random.
    pub fn update_spin(&mut self) {
        let (row, column) = self.random_position();
        let index = self.index(row, column).unwrap();

        let neighbor_sum = self.calc_neighbor_sum(row, column);
        let delta_energy = self.calc_delta_energy(row, column, neighbor_sum);

        if delta_energy < 0.0 || f64::exp(-delta_energy) > Math::random() {
            self.flip_spin(row, column);
        }
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(width: u32, height: u32, coupling: f64, magnetic_field: f64) -> Simulation {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let mut spins = FixedBitSet::with_capacity(size);

        for i in 0..spins.len() {
            spins.set(i, Math::random() >= SPIN_DOWN_PROBABILITY);
        }

        Simulation {
            width,
            height,
            coupling,
            magnetic_field,
            spins,
        }
    }

    /// Return pointer to the start of the spins collection.
    pub fn spins(&self) -> *const u32 {
        self.spins.as_slice().as_ptr()
    }

    pub fn update_spins(&mut self) {
        for i in 0..10000 {
            self.update_spin();
        }
    }

    pub fn magnetization(&self) -> f64 {
        let num_spin_up = self.spins.count_ones(..) as f64;
        let num_spins = (self.width * self.height) as f64;

        (2.0 * num_spin_up) / num_spins - 1.0
    }
}
