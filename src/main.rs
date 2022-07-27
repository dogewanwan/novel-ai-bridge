use actix_web::{post, web, App, HttpServer};
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;

mod error;
mod krake_client;

use krake_client::KrakeClient;

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub text: String,
}

async fn generate(
    app_state: &AppState,
    request: &GenerateRequest,
) -> Result<GenerateResponse, Box<dyn Error>> {
    info!("Got request: {:?}", request);

    let text = app_state.krake_client.generate(&request.text).await?;

    info!("Response from AI: {:?}", text);

    Ok(GenerateResponse { text })
}

#[post("/v1/krake/generate")]
async fn krake_generate(
    request: web::Json<GenerateRequest>,
    state: web::Data<AppState>,
) -> Result<web::Json<GenerateResponse>, Box<dyn Error>> {
    let response = generate(&state, &request).await?;
    Ok(web::Json(response))
}

pub struct AppState {
    pub krake_client: KrakeClient,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::try_init()?;

    let bearer = std::env::var("KRAKE_BEARER")?;
    let krake_client = KrakeClient::new("tokenizers/pile_tokenizer.json", &bearer)?;

    let client = web::Data::new(AppState { krake_client });

    HttpServer::new(move || App::new().app_data(client.clone()).service(krake_generate))
        .bind(("0.0.0.0", 7337))?
        .run()
        .await?;

    Ok(())
}
