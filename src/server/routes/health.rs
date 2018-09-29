use super::CorgiState;

use actix_web::{Json, Result};

#[derive(Serialize)]
pub struct HealthResponse {
  pub status: String
}

pub fn health(_: CorgiState) -> Result<Json<HealthResponse>> {
  Ok(Json(HealthResponse { status: "Ok".into() }))
}
