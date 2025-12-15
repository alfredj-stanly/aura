use ntex::web::{self, HttpResponse};

use crate::{
    agent::{LocalAgent, Agent},
    api::{AnalyzeRequest, FuzzyResponse, QueryParams, metrics::build_metrics},
    core::InferenceInput,
};

pub async fn handler(
    body: web::types::Json<AnalyzeRequest>,
    query: web::types::Query<QueryParams>,
) -> HttpResponse {
    let input = InferenceInput {
        email: body.email.clone(),
        name: body.name.clone(),
        profile_pic_url: body.profile_pic_url.clone(),
        browsing_history: body.browsing_history.clone(),
    };

    let local = LocalAgent::new();
    let analysis = local.analyze(&input).await;
    let include_metrics = !query.minimal.unwrap_or(false);

    let response = FuzzyResponse::from(analysis.clone())
        .with_metrics_if(include_metrics, || build_metrics(&analysis, &input));

    HttpResponse::Ok().json(&response)
}
