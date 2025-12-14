pub mod io;
pub mod local;
pub mod metrics;
pub mod r#type;

pub use io::*;

pub use metrics::*;
pub use r#type::*;

pub trait Agent: Send + Sync {
    fn analyze(
        &self,
        input: &InferenceInput,
    ) -> impl std::future::Future<Output = InferenceSignal> + Send;
}
