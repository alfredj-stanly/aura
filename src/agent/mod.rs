pub mod local;
pub mod r#type;

pub mod metrics;
pub mod handler;

pub use r#type::*;
pub use metrics::*;

pub trait Agent: Send + Sync {
    fn analyze(
        &self,
        input: &InferenceInput,
    ) -> impl std::future::Future<Output = InferenceSignal> + Send;
}
