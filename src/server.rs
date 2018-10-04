use std::sync::Arc;

// use futures::future::Future;
use futures::future::{Future, result};
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

use ::actix::prelude::*;
use actix_web::{
  server::HttpServer, http::Method, HttpRequest, AsyncResponder, HttpResponse, FutureResponse, App, State, Json,
  middleware::{Logger, cors::Cors}
};

use super::errors::*;

// mod routes;
// pub mod render;

use super::graphql as GraphQL;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
  type Result = Result<String>;
}

pub struct GraphQLExecutor {
  schema: Arc<GraphQL::Schema>
}

impl GraphQLExecutor {
  fn new(schema: Arc<GraphQL::Schema>) -> GraphQLExecutor {
    GraphQLExecutor { schema: schema }
  }
}

impl Actor for GraphQLExecutor {
  type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
  type Result = Result<String>;

  fn handle(&mut self, msg: GraphQLData, _ctx: &mut Self::Context) -> Self::Result {
    let res = msg.0.execute(&self.schema, &());
    let res_text = serde_json::to_string(&res)?;
    Ok(res_text)
  }
}

pub struct AppState {
  executor: Addr<GraphQLExecutor>
}
pub type CorgiState = State<Arc<AppState>>;

fn graphiql(_req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
  let html = graphiql_source("http://localhost:8080/graphql");
  result(
    Ok(
      HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
    )
  ).responder()
}

fn graphql((state, data): (State<AppState>, Json<GraphQLData>)) -> FutureResponse<HttpResponse> {
  state.executor
    .send(data.0)
    .from_err()
    .and_then(|res| match res {
      Ok(gql_data) => {
        Ok(HttpResponse::Ok().content_type("application/json").body(gql_data))
      },
      Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
}

/// Start the Corgi server.
///
pub fn start(address: &str) -> Result<()> {

  let schema = Arc::new(GraphQL::create_schema());
  let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

  let corgi_app = move || {
    App::with_state(AppState { executor: addr.clone() })
      .middleware(Logger::default())
      .configure(|app| {
        Cors::for_app(app)
          .allowed_origin("http://localhost:8080")
          .allowed_origin("http://127.0.0.1:8080")
          .resource("/graphql", |r| r.method(Method::POST).with(graphql))
          .register()
      })
      .resource("/graphiql", |r| r.method(Method::GET).h(graphiql))
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
