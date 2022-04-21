#!/usr/bin/env bash

export RUST_LOG="debug"
export DB_CONN="postgresql://postgres:Passw0rd@localhost:5432/tests"

cargo run
