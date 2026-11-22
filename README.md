# Lingua Omnibus (LO)

A free, open, community-governed programming language that lets programmers write code in their native human language while producing identical semantic output.

## Principles

Lingua Omnibus is governed by 13 non-negotiable principles (see PARTEA I / spec/00-conventions.md):

1. Zero vendor lock-in
2. Spec-driven, not implementation-driven
3. Reproducible builds
4. Bootstrappable toolchain
5. Federated packages
6. Offline-first
7. Zero telemetry
8. Open formats
9. Foundation governance (no BDFL)
10. Apache 2.0 license
11. Accessible hardware (32-bit, 512 MB RAM)
12. Free learning materials
13. LTS & migration

## Repository structure

```
spec/            # Normative specification (SPEC-00 … SPEC-13)
rfcs/            # Change proposals
impl/            # Reference implementation (loc, lopm, lolsp, lofmt, lodoc, lotest, runtime)
langpacks/       # Language packs (en, ro, fr, ...)
stdlib/          # Standard library written in LO
tests/           # Conformance, multilingual, differential, benchmarks
examples/        # Canonical demo programs
docs/            # The LO Book, tutorials, guides
governance/      # CHARTER, CONTRIBUTING, CODE_OF_CONDUCT, MAINTAINERS, TRADEMARK
ci/              # Reproducible build, bootstrap chain, conformance gate
```

## Getting started

1. Read `spec/00-conventions.md` (SPEC-00).
2. Write code in your language: create a `.lo` file starting with `#lingua: XX`.
3. Build with `loc build <file>.lo`.

## License

Apache 2.0 — see LICENSE.

## Created by

Claudiu Radu Groza
