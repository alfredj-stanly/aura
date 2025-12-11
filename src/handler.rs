use ntex::web::{self, HttpResponse};

use crate::model::{AgeBucketDistribution, GenderDistribution, InferRequest, InferResponse};
use crate::openai::OpenAIClient;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("As strong as an Ox!")
}

pub async fn infer(
    body: web::types::Json<InferRequest>,
    openai: web::types::State<OpenAIClient>,
) -> HttpResponse {
    // Extract Organisation
    let organization = body.email.split('@').nth(1).map(|s| s.to_string());

    // Call openai for inferense
    let result = openai.infer(&body.name, &body.email).await;

    match result {
        Ok(inference) => {
            let response = InferResponse {
                gender: GenderDistribution {
                    male: inference.gender.male,
                    female: inference.gender.female,
                    others: inference.gender.others,
                },
                age_bucket: AgeBucketDistribution {
                    age_18_24: inference.age_bucket.age_18_24,
                    age_25_34: inference.age_bucket.age_25_34,
                    age_35_34: inference.age_bucket.age_35_44,
                    age_45_plus: inference.age_bucket.age_45_plus,
                },

                organization,
                confidence: inference.confidence,
                region_hint: inference.region_hint,
                edge_case: false,
            };

            HttpResponse::Ok().json(&response)
        }
        Err(e) => {
            eprintln!("Inference failed from open-ai side: {e}");
            HttpResponse::InternalServerError().body(format!("inference failed: {}", e))
        }
    }
}
