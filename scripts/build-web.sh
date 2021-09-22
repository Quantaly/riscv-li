#!/bin/bash
set -eux

cd web
wasm-pack build --target web
