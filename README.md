# actix-playground

Just playing around with actix-web, and seeing what I can do with it.

## Running the server

```bash
# Install diesel_cli with sqlite support
cargo install diesel_cli --no-default-features --features sqlite
# Install cargo-watch for auto-reloading
cargo install cargo-watch
# Setup the database
diesel setup
# Run the server
cargo watch -x run
```
