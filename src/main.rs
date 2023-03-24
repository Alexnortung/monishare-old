use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

mod router;

#[derive(Clone)]
pub struct AppState {
    db: edgedb_tokio::Client,
}

// #[derive(Deserialize)]
// struct RegisterWithEmail {
//     email: String,
//     password: String,
// }
//
// #[derive(Deserialize)]
// struct RegisterWithUsername {
//     username: String,
//     password: String,
// }
//
// #[debug_handler]
// async fn register(
//     State(state): State<Arc<AppState>>,
//     Json(body): Json<RegisterWithEmail>,
// ) -> Result<String, StatusCode> {
//     let db = &state.db;
//     let val = match db
//         .query_required_single::<i64, _>("select 5 + 6", &())
//         .await
//     {
//         Ok(val) => val,
//         Err(e) => {
//             return Err(StatusCode::INTERNAL_SERVER_ERROR);
//         }
//     };
//     Ok(val.to_string())
// }

#[tokio::main]
async fn main() {
    // get edgedb db connections
    let db = match edgedb_tokio::create_client().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error connecting to EdgeDB: {}", e);
            std::process::exit(1);
        }
    };

    let shared_state = AppState { db };

    // build our application with a single route
    let app: axum::Router = Router::new()
        // .route("/", get(|| async { "Hello, World!" }))
        .nest("/", router::root::root_router())
        .with_state(shared_state);

    // run it with hyper on localhost:3000
    println!("Listening on port 3001");
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
