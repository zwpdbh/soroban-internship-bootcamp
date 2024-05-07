# How to run

This is for "Submit your Pre-work Project"

```sh
cargo run 
```

## Note of Soroban internship bootcamp

### Install Soroban CLI

- Run `cargo install --locked soroban-cli`.
  - If meet compile error, switch to use stable rust (not nightly).
- To configure your CLI to interact with Testnet, run the following command:

  ```sh
  soroban contract init . 
  ```