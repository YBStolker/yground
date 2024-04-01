#!/bin/bash
cargo watch -cs "npx tailwindcss -i ./input.css -o ./public/styles.css && cargo run"
