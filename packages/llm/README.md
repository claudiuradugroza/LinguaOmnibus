# llm — Tiny LLM package for Lingua Omnibus

A small, educational LLM (inference + greedy decoding) package for
Lingua Omnibus, inspired by [llm.asm](https://github.com/alesha-pro/llm.asm).

## Status

- High-level orchestration is written in Lingua Omnibus (`.lo`).
- Heavy kernels (matmul, attention, RMSNorm, RoPE, softmax, BPE tokenizer)
  live in `kernels/kernels.c` as a reference and will be replaced by
  hand-written x86-64 assembly in a future release (see `llm.asm`).
- Lingua Omnibus does not yet have FFI, SIMD intrinsics or manual memory
  control, so this package is currently **type-checked and demonstrative**.
  It passes `loc check` for every `.lo` file.

## Layout

```
packages/llm/
  loc.toml
  README.md
  src/
    tensor.lo      # vector/matrix helpers
    model.lo       # config + param count
    tokenizer.lo   # char-level tokenizer + argmax
    inference.lo   # greedy decoding loop
    main.lo        # entry point
  examples/
    generate.lo
  tests/
    smoke_test.lo
  kernels/
    kernels.c      # reference C kernels (AVX2/asm to come)
```

## Build / check

```sh
# Type-check all .lo files
for f in packages/llm/src/*.lo packages/llm/examples/*.lo packages/llm/tests/*.lo; do
  loc check "$f"
done

# Compile kernels
cc -O3 -march=native -c packages/llm/kernels/kernels.c -o /tmp/kernels.o
```

## Roadmap to a real LLM in LO

1. FFI (call C/asm kernels from `.lo`).
2. SIMD intrinsics / inline asm in the native backend.
3. Pointer/arena types and manual memory control.
4. LLVM backend for real optimizations.
5. `std.math` with exp/sqrt and `std.io` file reading.

Then this package can host a full Qwen3-0.6B-style inference engine.