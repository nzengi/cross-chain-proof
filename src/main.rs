use std::sync::{Arc, Mutex};
use cross_chain_merkle_proof::{blockchain::Blockchain, api};

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state = Arc::new(Mutex::new((Blockchain::new(), None)));

    // Build the API
    let api_routes = api::api(state);

    // Start the server
    warp::serve(api_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
