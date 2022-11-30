set shell := ["powershell.exe", "-c"]

run:
    cargo run --release

dev:
    cargo run

test:
    cargo test
