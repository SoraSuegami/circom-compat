//! Arkworks - Circom Compatibility layer
//!
//! Provides bindings to Circom's R1CS, for Groth16 Proof and Witness generation in Rust.
#[cfg(not(target_arch = "wasm32"))]
mod witness;
#[cfg(not(target_arch = "wasm32"))]
pub use witness::WitnessCalculator;

pub mod circom;
#[cfg(not(target_arch = "wasm32"))]
pub use circom::{CircomBuilder, CircomConfig};
pub use circom::{CircomCircuit, CircomReduction};

#[cfg(feature = "ethereum")]
pub mod ethereum;

mod zkey;
pub use zkey::read_zkey;
