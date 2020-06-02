//! Utility functions to simplify the code.

use js_sys::Math;

/// Enable panic output forwarded to the web console.
///
/// Requires the `console_error_panic_hook` feature to be enabled.
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Generate a random u32 up to a given maximum.
pub fn random_u32_up_to(max: u32) -> u32 {
    let max = max as f64;
    (Math::random() * max) as u32
}
