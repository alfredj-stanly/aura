use ntex::web::{self, HttpResponse};

use crate::model::{AgeBucketDistribution, GenderDistribution, InferRequest, InferResponse};

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("As strong as an Oak!")
}

pub async fn infer(body: web::types::Json<InferRequest>) -> HttpResponse {
    println!("Recived request for: {}", body.name);

    // Extract Organisation
    let organization = body.email.split('@').nth(1).map(|s| s.to_string());

    let response = InferResponse {
        gender: GenderDistribution {
            male: 0.50,
            female: 0.45,
            others: 0.05,
        },
        age_bucket: AgeBucketDistribution {
            age_18_24: 0.05,
            age_25_34: 0.65,
            age_35_34: 0.25,
            age_45_plus: 0.03
        },

        organization,
        confidence: 0.0,
        edge_case: true,
    };

    HttpResponse::Ok().json(&response)
}
