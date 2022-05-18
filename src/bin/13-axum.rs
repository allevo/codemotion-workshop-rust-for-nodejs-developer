
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    extract::{Extension, FromRequest, RequestParts},
    Json, Router,
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use mongodb::{Client, options::ClientOptions, Collection, bson::oid::ObjectId};
use futures_util::TryStreamExt;
use futures_util::StreamExt;

// Rust doens't have async support by default:
// Rust provides an interface to async runtime
// Tokio is one of the famous runtime
// The following line start the `main` in async way
#[tokio::main]
async fn main() {
    // We would like to have the logs
    // Tracing (technically is more then just log lib) is
    // a way to have a log system
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            // We can use the environment variable `RUST_LOG`
            // to configure at runtime the log level.
            // The log level can be defined for module
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "13_tide=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let employees_collection = connect_to_mongodb().await;

    let router = Router::new()
        // add routes to router registering:
        // - GET /employees -> fetch all employees
        // - POST /employees -> create one employee
        .route("/employees", get(get_users).post(create_user))
        // Logs incoming request and outcoming response
        .layer(TraceLayer::new_for_http())
        // Add state
        .layer(Extension(employees_collection));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

// Handler
async fn get_users(
    // We can access to the handler in this way
    Extension(employees_collection): Extension<Collection<EmployeeEntity>>,
) -> impl IntoResponse  {
    let all_employees: Vec<EmployeeDTO> = employees_collection.find(None, None)
        .await
        .unwrap()
        // We map EmployeeEntity to EmployeeDTO
        // We use `.into` in order to not making the mapping here
        // So here we are focused on business logic, not on the mapping one
        .map(|entity: Result<EmployeeEntity, mongodb::error::Error>| entity.unwrap().into())
        .collect()
        .await;

    (StatusCode::OK, Json(all_employees))
}

// Handler
async fn create_user(
    Extension(employees_collection): Extension<Collection<EmployeeEntity>>,
    Json(input): Json<EmployeeCreation>,
) -> impl IntoResponse  {
    // Creating the entity
    let entity = EmployeeEntity {
        // MongoDb uses ObjectId as type of PK
        _id:  ObjectId::new(),
        name: input.name,
        salary: input.salary,
        age: input.age,
    };

    // inserts
    let res = employees_collection.insert_one(entity, None).await.unwrap();
    let id = res.inserted_id.as_object_id().unwrap().to_hex();

    // returns the id
    (StatusCode::CREATED, Json(id))
}

#[derive(Deserialize)]
struct EmployeeCreation {
    name: String,
    salary: usize,
    age: usize,
}
// The entity
#[derive(Serialize, Deserialize)]
struct EmployeeEntity {
    _id: ObjectId,
    name: String,
    salary: usize,
    age: usize,
}

// The entity
#[derive(Serialize)]
struct EmployeeDTO {
    id: String,
    name: String,
    salary: usize,
    age: usize,
}

// This is the implementation for mapping the entity into DTO
impl From<EmployeeEntity> for EmployeeDTO {
    fn from(e: EmployeeEntity) -> Self {
        Self {
            id: e._id.to_hex(),
            name: e.name,
            salary: e.salary,
            age: e.age,
        }
    }
}

// Connects to mongo and returns the connection
async fn connect_to_mongodb() -> Collection<EmployeeEntity> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017/").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("mydb");

    db.collection::<EmployeeEntity>("employees")
}
