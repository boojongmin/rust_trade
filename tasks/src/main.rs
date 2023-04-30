use std::{env, pin::Pin, time::Duration, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Error
};

use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use juniper_actix::{graphql_handler, playground_handler};
use tokio::io;

// type Schema = RootNode<'static, Query, EmptyMutation<Database>, Subscription>;

// fn schema() -> Schema {
//     Schema::new(Query, EmptyMutation::<Database>::new(), Subscription)
// }

mod schema;

use crate::schema::{create_schema, Schema};

mod db;
use crate::db::DB;




async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let db = schema::Database{};
    let user = data.execute(&st, &db).await;
    HttpResponse::Ok().json(user)
}

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = schema::Database{};
    graphql_handler(&schema, &context, req, payload).await
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");
    log::info!("GraphiQL playground: http://localhost:8080/playground");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .service(web::resource("/playground").route(web::get().to(playground)))
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
