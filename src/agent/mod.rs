pub mod local;
pub use local::*;

use crate::core::{InferenceSignal, InferenceInput}; 


pub trait Agent: Send + Sync {
    fn analyze(
        &self,
        input: &InferenceInput,
    ) -> impl std::future::Future<Output = InferenceSignal> + Send;
}
