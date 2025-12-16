use ntex::web::{self, HttpResponse};

use crate::{
    agent::{Agent, LocalAgent, VisionAgent},
    api::{AnalyzeRequest, InferResponse, QueryParams, metrics::build_metrics},
    core::{InferenceInput, fuse},
};

pub async fn handler(
    body: web::types::Json<AnalyzeRequest>,
    query: web::types::Query<QueryParams>,
    api_key: web::types::State<String>,
) -> HttpResponse {
    let input = InferenceInput {
        email: body.email.clone(),
        name: body.name.clone(),
        profile_pic_url: body.profile_pic_url.clone(),
        browsing_history: body.browsing_history.clone(),
    };

    let local = LocalAgent::new();
    let mut signals = vec![local.analyze(&input).await];

    // Call vision agent if image is provided.
    if input.profile_pic_url.is_some() {
        let vision = VisionAgent::new(api_key.get_ref().clone());
        signals.push(vision.analyze(&input).await);
    }

    let fused = fuse(signals.clone());

    let include_metrics = !query.minimal.unwrap_or(false);
    let response = InferResponse::from(fused.clone())
        .with_metrics_if(include_metrics, || build_metrics(&signals, &fused, &input));

    HttpResponse::Ok().json(&response)
}
