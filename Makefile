deploy: 
	soroban contract deploy --wasm target/wasm32-unknown-unknown/release/hello_world.wasm --source alice --network testnet