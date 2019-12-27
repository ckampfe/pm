#!/usr/bin/env sh

wasm-pack build --release --target web \
  && rollup ./main.js --format iife --file ./pkg/bundle.js
