use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

mod pool;
mod transaction;

use pool::PoolManager;

#[derive(Clone)]
struct AppState {
    pool_manager: Arc<Mutex<PoolManager>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct PoolInfo {
    theme: String,
    reserve_a: u64,
    reserve_b: u64,
    fee_basis_points: u16,
}

#[derive(Serialize, Deserialize)]
struct SwapRequest {
    amount_in: u64,
    min_amount_out: u64,
}

#[derive(Serialize, Deserialize)]
struct SwapResponse {
    amount_out: u64,
    price_impact: f64,
}

#[derive(Serialize, Deserialize)]
struct LiquidityRequest {
    amount_a: u64,
    amount_b: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool_manager = Arc::new(Mutex::new(PoolManager::new()));
    let state = AppState {
        pool_manager: pool_manager.clone(),
    };

    // Initialize orchestral sections
    let mut pm = pool_manager.lock().await;
    pm.initialize_pool("Violins", 30);
    pm.initialize_pool("Cellos", 30);
    pm.initialize_pool("Violas", 30);
    drop(pm);

    let app = Router::new()
        .route("/health", get(health))
        .route("/pool/:theme", get(get_pool))
        .route("/swap/:theme", post(swap))
        .route("/liquidity/:theme", post(add_liquidity))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("🎻 Concertino AMM Backend running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "🎼 Harmonious" }))
}

async fn get_pool(
    Path(theme): Path<String>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<PoolInfo>, StatusCode> {
    let pm = state.pool_manager.lock().await;
    match pm.get_pool(&theme) {
        Some(info) => Ok(Json(info)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn swap(
    Path(theme): Path<String>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<SwapRequest>,
) -> Result<Json<SwapResponse>, StatusCode> {
    let mut pm = state.pool_manager.lock().await;
    match pm.calculate_swap(&theme, req.amount_in) {
        Some(amount_out) => {
            if amount_out < req.min_amount_out {
                return Err(StatusCode::PRECONDITION_FAILED);
            }
            let price_impact = 1.0 - (amount_out as f64 / req.amount_in as f64);
            Ok(Json(SwapResponse {
                amount_out,
                price_impact,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn add_liquidity(
    Path(theme): Path<String>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<LiquidityRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut pm = state.pool_manager.lock().await;
    match pm.add_liquidity(&theme, req.amount_a, req.amount_b) {
        Some(lp_amount) => Ok(Json(serde_json::json!({
            "lp_tokens": lp_amount,
            "message": "Maestro joined the orchestra!"
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
