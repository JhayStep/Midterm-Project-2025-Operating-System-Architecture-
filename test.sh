#!/usr/bin/env bash
set -euo pipefail

echo "[1/3] cargo build"
cargo build

echo "[2/3] run simple_test.lisp"
OUT1=$(cargo run --quiet -- tests/simple_test.lisp | tail -n 1)
echo "simple_test.lisp => $OUT1"
if [ "$OUT1" != "10" ]; then
  echo "Expected 10, got $OUT1"
  exit 1
fi

echo "[3/3] run test2.lisp"
OUT2=$(cargo run --quiet -- tests/test2.lisp | tail -n 1)
echo "test2.lisp => $OUT2"
if [ "$OUT2" != "9" ]; then
  echo "Expected 9, got $OUT2"
  exit 1
fi

echo "[PASS] all tests"
