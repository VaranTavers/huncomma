mod typical;
mod pair;
mod naive;
mod naive_forward;

pub use naive::NaiveDetector;
pub use naive_forward::NaiveForwardDetector;
pub use pair::PairDetector;
pub use typical::TypicalDetector;