pub mod local;
pub mod vision;

pub use local::*;
pub use vision::VisionAgent;

use crate::core::{InferenceInput, InferenceSignal};

pub trait Agent: Send + Sync {
    fn analyze(
        &self,
        input: &InferenceInput,
    ) -> impl std::future::Future<Output = InferenceSignal> + Send;
}
