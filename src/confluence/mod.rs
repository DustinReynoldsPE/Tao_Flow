pub mod decomposition;
pub mod detection;
pub mod eddy;
pub mod pool;
pub mod yielding;

pub use decomposition::Decomposer;
pub use eddy::{Eddy, EddyNature, Position, Resolution};
pub use pool::ConfluencePool;
