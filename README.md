# CrossWordle

## Frontend

Navigate to the `bevy_frontend` directory

### Desktop
Use the following command to run the front end locally (aka as a rust desktop application):
```
cargo run
```

### Web

First ensure you have the necessary dependencies installed:
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
```
Then, use the following command to run the front end as a wasm binary (aka for web development):
```
cargo run --target wasm32-unknown-unknown
```