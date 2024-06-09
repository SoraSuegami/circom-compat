use ark_ec::pairing::Pairing;

pub mod r1cs_reader;
pub use r1cs_reader::{R1CSFile, R1CS};

mod circuit;
pub use circuit::CircomCircuit;

#[cfg(not(target_arch = "wasm32"))]
mod builder;
#[cfg(not(target_arch = "wasm32"))]
pub use builder::{CircomBuilder, CircomConfig};

mod qap;
pub use qap::CircomReduction;

pub type Constraints<E> = (ConstraintVec<E>, ConstraintVec<E>, ConstraintVec<E>);
pub type ConstraintVec<E> = Vec<(usize, <E as Pairing>::ScalarField)>;
