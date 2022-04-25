#!/usr/bin/env bash

export RUST_LOG="info"
export DB_CONN="postgresql://postgres:Passw0rd@localhost:5432/tests"
#export RUST_BACKTRACE=1

cargo run -- --sleep 1000 --querytype 2
