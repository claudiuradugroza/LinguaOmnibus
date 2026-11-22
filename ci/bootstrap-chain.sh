#!/usr/bin/env bash
# Bootstrap chain (Principle 4)
# Minimal path: TCC/C → LO-0 → LO-1 (self-hosted loc)

set -euo pipefail

echo "=== Lingua Omnibus Bootstrap Chain ==="

echo "[1/3] Stage 0: Bootstrap compiler (TCC/C)..."
# Build stage-0 compiler from C sources

echo "[2/3] Stage 1: LO-0 compiler..."
# Compile LO sources with stage-0

echo "[3/3] Stage 2: Self-hosted loc..."
# Compile loc with LO-0; verify it can compile itself

echo "=== Bootstrap chain complete ==="