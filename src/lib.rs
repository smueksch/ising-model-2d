use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;
use js_sys::Math;

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

const SPIN_DOWN_PROBABILITY: f64 = 0.3;

/*
 * Simulation
 */

#[wasm_bindgen]
struct Simulation {
    width: u32,
    height: u32,
    spins: FixedBitSet,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(width: u32, height: u32) -> Simulation {
        let size = (width * height) as usize;
        let mut spins = FixedBitSet::with_capacity(size);

        for i in 0..spins.len() {
            spins.set(i, Math::random() >= SPIN_DOWN_PROBABILITY);
        }
        
        Simulation {
            width,
            height,
            spins,
        }
    }

    /// Return pointer to the start of the spins collection.
    pub fn spins(&self) -> *const u32 {
        self.spins.as_slice().as_ptr()
    }
}
