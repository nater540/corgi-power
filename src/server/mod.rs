use std::sync::Arc;

use futures::future::Future;
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

use actix::{Message, SyncContext, SyncArbiter, Addr, Actor, Context};
// use ::actix::prelude::*;

use actix_web::{
  server::HttpServer, http::Method, HttpRequest, HttpResponse,
  App, AsyncResponder, FutureResponse, State, Json, dev::Handler
};

use super::errors::*;

mod routes;
pub mod render;

mod graphql;
use self::graphql::Schema as Schema;

pub struct AppState {
  executor: Addr<GraphQLExecutor>
}

pub type CorgiState = State<Arc<AppState>>;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
  type Result = Result<String>;
}

pub struct GraphQLExecutor {
  schema: Arc<Schema>
}

impl GraphQLExecutor {
  fn new(schema: Arc<Schema>) -> GraphQLExecutor {
    GraphQLExecutor { schema: schema }
  }
}

impl Actor for GraphQLExecutor {
  type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
  type Result = Result<String>;

  fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
    let res = msg.0.execute(&self.schema, &());
    let res_text = serde_json::to_string(&res)?;
    Ok(res_text)
  }
}

// fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, actix_web::Error> {
//   let html = graphiql_source("http://127.0.0.1:8080/graphql");
//   Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
// }

// fn graphql((state, data): (State<AppState>, Json<GraphQLData>)) -> FutureResponse<HttpResponse> {
//   state.executor
//     .send(data.0)
//     .from_err()
//     .and_then(|res| match res {
//       Ok(gql_data) => {
//         Ok(HttpResponse::Ok().content_type("application/json").body(gql_data))
//       },
//       Err(_) => Ok(HttpResponse::InternalServerError().into()),
//     })
//     .responder()
// }

/// Start the Corgi server.
///
pub fn start(address: &str) -> Result<()> {

  let schema = Arc::new(graphql::create_schema());
  let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

  let corgi_app = move || {
    App::with_state(AppState { executor: addr.clone() })
      // .route("/", Method::GET, routes::index)
      // .resource("/health", |res| {
      //   res.method(Method::GET)
      //     .with(routes::health)
      // })
  };

  info!("Server running on {}", address);
  HttpServer::new(corgi_app)
    .bind(address)
    .unwrap()
    .start();

  Ok(())
}
