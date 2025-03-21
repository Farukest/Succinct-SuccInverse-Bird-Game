# Succinct (Flappy) Bird Zero-Knowledge Proof API

This project implements a **REST API** using **Axum** and **SP1 SDK** to generate **zero-knowledge proofs** for a **Flappy Bird** game simulation. It processes game inputs (**flaps and username**) and returns a **Groth16 proof** with score and username verification.

---

## Game Play and Prove Operation

![Flappy ZK Proof Demo](proveandplay.gif)

---

## 🚀 Features

- **Game Simulation:** Simulates Flappy Bird gameplay in `program.rs` using `flaps` input.
- **Proof Generation:** Generates a **Groth16 proof** with **SP1 SDK**, committing **score and username**.
- **API:** Axum server at `0.0.0.0:3000/api/prove-score` returns proof, public inputs, verifying key hash, score, and username in hex format.
- **Docker Integration:** Uses **Docker** for Groth16 proof generation.

---

## 🛠 Setup

### **1️⃣ Install Dependencies**
```bash
cargo build --release
sudo apt install -y docker.io
sudo systemctl start docker
```

### **2️⃣ Run the API Server**
```bash
RUST_LOG=info cargo run --release --bin main
```

---

## 📡 Usage

### **Send a POST request:**
```javascript
fetch('http://localhost:3000/api/prove-score', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ flaps: [39, 90, 110], username: "player1" })
}).then(res => res.json()).then(data => console.log(data));
```

---

## 📜 Response Example
```json
{
  "proof": "11b6a09d...",
  "public_inputs": "01000000...",
  "vkey_hash": "0x005c3cf5...",
  "mode": "groth16",
  "score": 1,
  "username": "player1"
}
```

---

## ⚠ Notes

- **Groth16 proof generation takes ~42s due to SNARK wrapping.**
- **Requires Docker for Gnark backend.**

## Its based on Succinct Floppy GPU

## Succinct SP1 Game