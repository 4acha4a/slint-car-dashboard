use axum::{Json, Router, extract::State, routing::{get, post}};
use crate::{vehicle::VehicleData};
use std::sync::{Arc};
use tokio::sync::RwLock;
use std::net::SocketAddr;

type SharedData = Arc<RwLock<Option<VehicleData>>>;

pub async fn send_vehicle_data(
    State(data): State<SharedData>,
    Json(payload): Json<VehicleData>,
) -> Result<&'static str, ()> {
    let mut data_lock = data.write().await;
    *data_lock = Some(payload);

    Ok("Vehicle data updated")
}

async fn get_vehicle_data(
    State(data): State<SharedData>,
) -> Json<Option<VehicleData>> {
    let data_lock = data.read().await;

    Json(data_lock.clone())
}

pub async fn run_web() {
    println!("Starting web server on http://127.0.0.1:3000");
    let shared_data: SharedData = Arc::new(RwLock::new(None));
    let app = Router::new()
        .route("/data", post(send_vehicle_data))
        .route("/data", get(get_vehicle_data))
        .with_state(shared_data);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}