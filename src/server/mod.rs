use std::sync::Arc;

use actix_web::{
  server::HttpServer, http::Method, App, State
};

use super::errors::*;

mod routes;
pub mod render;

#[derive(Debug)]
pub struct AppState {
  pub settings: String
}

pub type CorgiState = State<Arc<AppState>>;

/// Start the Corgi server.
///
pub fn start(address: &str) -> Result<()> {
  let state = Arc::new(AppState {
    settings: "Kitty!".into()
  });

  let corgi_app = move || {
    App::with_state(state.clone())
      .route("/", Method::GET, routes::index)
      .resource("/health", |res| {
        res.method(Method::GET)
          .with(routes::health)
      })
  };

  HttpServer::new(corgi_app)
    .bind(address)
    .unwrap()
    .start();

  Ok(())
}
