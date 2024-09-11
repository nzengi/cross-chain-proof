use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::merkle_tree::MerkleTree;

// Data structures
#[derive(Deserialize, Serialize)]
struct MerkleTreeData {
    leaves: Vec<Vec<u8>>,
}

#[derive(Deserialize, Serialize)]
struct BlockchainData {
    root_hash: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
struct ProofData {
    proof: Vec<Vec<u8>>,
    leaf: Vec<u8>,
}

#[derive(Serialize)]
struct ProofResponse {
    verified: bool,
}

// Shared state for blockchain and Merkle tree
type State = Arc<Mutex<(Blockchain, Option<MerkleTree>)>>;

pub fn api(state: State) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_merkle_tree = warp::path("merkle_tree")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_create_merkle_tree);

    let add_root = warp::path("add_root")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_add_root);

    let verify_proof = warp::path("verify_proof")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_verify_proof);

    create_merkle_tree.or(add_root).or(verify_proof)
}

fn with_state(state: State) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_create_merkle_tree(data: MerkleTreeData, state: State) -> Result<impl warp::Reply, warp::Rejection> {
    let merkle_tree = MerkleTree::new(data.leaves);
    let mut state_lock = state.lock().unwrap();
    state_lock.1 = Some(merkle_tree);

    let root_hash = state_lock.1.as_ref().unwrap().root_hash().unwrap();
    Ok(warp::reply::json(&root_hash))
}

async fn handle_add_root(data: BlockchainData, state: State) -> Result<impl warp::Reply, warp::Rejection> {
    let mut state_lock = state.lock().unwrap();
    let blockchain = &mut state_lock.0;
    blockchain.add_root(1, data.root_hash);

    Ok(warp::reply::json(&"Root hash added"))
}

async fn handle_verify_proof(data: ProofData, state: State) -> Result<impl warp::Reply, warp::Rejection> {
    let state_lock = state.lock().unwrap();
    let blockchain = &state_lock.0;
    let verified = blockchain.verify_proof(data.proof, data.leaf);

    Ok(warp::reply::json(&ProofResponse { verified }))
}
