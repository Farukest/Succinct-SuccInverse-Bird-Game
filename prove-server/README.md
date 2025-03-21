# Flappy Bird Zero-Knowledge Proof API

This project implements a REST API using Axum and SP1 SDK to generate zero-knowledge proofs for a Flappy Bird game simulation. It processes game inputs (flaps and username) and returns a Groth16 proof with score and username verification.

## 🚀 Features

- ** Game Simulation: Simulates Flappy Bird gameplay in program.rs using flaps input. **

- ** Proof Generation: Generates a Groth16 proof with SP1 SDK, committing score and username. **

- ** API: Axum server at 0.0.0.0:3000/api/prove-score returns proof, public inputs, verifying key hash, score, and username in hex format. **

- ** Docker Integration: Uses Docker for Groth16 proof generation. **

- ** Succinverse Integration: Fetches flaps input from the web-based Succinverse Game and simulates gameplay to generate a proof that cannot be manipulated. **

