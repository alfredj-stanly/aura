pub mod io;
pub mod local;
pub mod metrics;
pub mod r#type;

pub use io::*;

pub use metrics::*;
pub use r#type::*;

pub trait Agent {
    fn analyze(&self, input: &InferenceInput) -> InferenceSignal;
}
