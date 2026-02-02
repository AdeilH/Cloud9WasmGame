# League WASM Game

A top-down survival game built with **Bevy 0.15**, targeting **WebAssembly (WASM)**.

## 🚀 Features
- **Fast WASM Performance**: Optimized build pipeline using `Trunk` and `wasm-opt`.
- **3D Graphics**: Physically Based Rendering (PBR) using GLTF/GLB models.
- **Dynamic Gameplay**: Survival mechanics with enemy scaling and character selection.
- **Responsive UI**: Built-in Bevy UI for menus, loading screens, and HUD.
- **Docker Ready**: Includes configuration for containerized deployment with Nginx.

## 🛠 Tech Stack
- **Engine**: [Bevy 0.15](https://bevyengine.org/)
- **Language**: [Rust](https://www.rust-lang.org/)
- **Build Tool**: [Trunk](https://trunkrs.dev/)
- **Optimization**: [wasm-opt](https://github.com/WebAssembly/binaryen)
- **Deployment**: Docker & Nginx

## 🎨 Credits
- **Assets**: 3D models and textures by [Kenney](https://kenney.nl/assets).

## 🏃 How to Run Locally

### Prerequisites
- [Rust](https://rustup.rs/)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install --locked trunk`

### Running with Trunk
```bash
trunk serve
```
Open your browser at `http://localhost:4000`.

### Running with Docker
```bash
make run
```
Open your browser at `http://localhost:8080`.

## 📂 Project Structure
- `src/`: Rust source code.
  - `main.rs`: Entry point and plugin initialization.
  - `app.rs`: Core game logic, systems, and UI.
- `assets/`: 3D models, textures, and UI assets.
- `Trunk.toml`: Configuration for the Trunk build pipeline.
- `index.html`: Web entry point and asset staging.
- `nginx.conf`: Nginx configuration for serving WASM and GLB files.

## 🎮 Controls
- **Movement**: Cursor-based orientation.
- **Attack**: `Space` or `Left Mouse Button`.
- **Goal**: Survive for 5 minutes!
