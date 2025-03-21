use axum::{extract::Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin, HashableKey};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tokio::task;

const FLAPPY_BIRD_ELF: &[u8] = include_elf!("flappy-bird");

#[derive(Debug, Deserialize)]
struct GameData {
    flaps: Vec<u32>,
    username: String, // Username eklendi
}

#[derive(Debug, Serialize)]
struct ProofData {
    proof: String,
    public_inputs: String,
    vkey_hash: String,
    mode: String,
    score: u32,      // Skoru ayrı döndürelim
    username: String, // Username’i de döndürelim
}

async fn prove_score(Json(game_data): Json<GameData>) -> Json<ProofData> {
    println!("DEBUG: Received game data: {:?}", game_data);
    let mut stdin = SP1Stdin::new();
    println!("DEBUG: Writing flaps: {:?}", game_data.flaps);
    stdin.write(&game_data.flaps);
    println!("DEBUG: Writing username: {}", game_data.username);
    stdin.write(&game_data.username);
    println!("DEBUG: Loaded flappy bird ELF ({} bytes)", FLAPPY_BIRD_ELF.len());

    let blocking_result = task::spawn_blocking(move || -> Result<(Vec<u8>, String, Vec<u8>, u32, String), Box<dyn std::error::Error + Send>> {
        println!("DEBUG: [spawn_blocking] Starting proof generation");
        utils::setup_logger();
        let client = ProverClient::from_env();
        println!("DEBUG: [spawn_blocking] ProverClient initialized");
        let (pk, vk) = client.setup(FLAPPY_BIRD_ELF);
        println!("DEBUG: [spawn_blocking] Setup completed");
        let proof = client.prove(&pk, &stdin).groth16().run()?;
        println!("DEBUG: [spawn_blocking] Proof generated successfully");
        let proof_bytes = proof.bytes().to_vec();
        let mut public_values = proof.public_values;
        let score = public_values.read::<u32>(); // Skoru oku
        let username = public_values.read::<String>(); // Username’i oku
        let public_values_vec = public_values.to_vec();
        let vkey_hash = vk.bytes32();
        println!(
            "DEBUG: [spawn_blocking] Proof details: proof_bytes ({} bytes), public_inputs ({} bytes), vkey_hash: {}, score: {}, username: {}",
            proof_bytes.len(),
            public_values_vec.len(),
            vkey_hash,
            score,
            username
        );
        Ok((proof_bytes, vkey_hash, public_values_vec, score, username))
    })
    .await
    .map_err(|e| format!("Blocking task join error: {:?}", e));

    let (proof_bytes, vkey_hash, public_values_vec, score, username) = match blocking_result {
        Ok(Ok(tuple)) => {
            println!("DEBUG: [spawn_blocking] Task completed successfully");
            tuple
        }
        Ok(Err(e)) => {
            println!("DEBUG: [spawn_blocking] Task error: {:?}", e);
            return Json(ProofData {
                proof: String::new(),
                public_inputs: String::new(),
                vkey_hash: String::new(),
                mode: "error".to_string(),
                score: 0,
                username: String::new(),
            });
        }
        Err(e) => {
            println!("DEBUG: [spawn_blocking] Join error: {:?}", e);
            return Json(ProofData {
                proof: String::new(),
                public_inputs: String::new(),
                vkey_hash: String::new(),
                mode: "error".to_string(),
                score: 0,
                username: String::new(),
            });
        }
    };

    let proof_data = ProofData {
        proof: hex::encode(proof_bytes),
        public_inputs: hex::encode(public_values_vec),
        vkey_hash,
        mode: "groth16".to_string(),
        score,
        username,
    };

    println!("DEBUG: Successfully prepared proof JSON: {:?}", proof_data);
    Json(proof_data)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/api/prove-score", post(prove_score))
        .layer(cors);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Starting proof API server on 0.0.0.0:3000...");
    axum::serve(listener, app).await.unwrap();
}