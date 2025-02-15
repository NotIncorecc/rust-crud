# FastAPI Clone in Rust using Actix-web

This is a simple CRUD API built with **Rust** and **Actix-web**, similar to FastAPI.

## 🚀 Features
- Create an item (`POST /items`)
- Retrieve all items (`GET /items`)
- Update an existing item (`PUT /items`)
- Delete an item (`DELETE /items/{id}`)

---

## 🛠 Setup Instructions

### 1️⃣ Install Rust
Ensure you have Rust installed. If not, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### 2️⃣ Clone this repository
```sh
git clone https://github.com/your-username/fastapi_clone_rust.git
cd fastapi_clone_rust
```
### 3️⃣ Install Dependencies
Navigate to the project root and run:
```sh
cargo build
```
### 4️⃣ Run the Server
```sh
cargo run
```
The server starts at http://127.0.0.1:8080.