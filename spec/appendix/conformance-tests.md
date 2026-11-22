# Appendix — Conformance Tests

Define what every conforming implementation must pass. Test paths follow the normative pattern from CONV-012:

    conformance/{PREFIX-lowercase}/{NNN}/{slug}.lo

Each test file MUST declare covered rule identifiers in a header comment:

    # covers: CONV-001, CONV-002

## Minimum required set at spec-0.1 (CONV section)

| Rule | Test path |
|------|-----------|
| CONV-001, CONV-002 | conformance/conv/001/keywords-allcaps.lo |
| CONV-010, CONV-011 | conformance/conv/010/rule-ids.lo |
| CONV-012, CONV-013 | conformance/conv/012/test-path-mapping.lo |
| CONV-020, CONV-021 | conformance/conv/020/reserved-prefixes.lo |
| CONV-030 … CONV-034 | conformance/conv/030/version-declaration.lo |
| CONV-040 … CONV-042 | conformance/conv/040/conforming-impl.lo |
| CONV-050 … CONV-052 | conformance/conv/050/deprecation.lo |
| CONV-060 … CONV-064 | conformance/conv/060/langpack-contract.lo |
| CONV-070 … CONV-073 | conformance/conv/070/unicode-baseline.lo |
| CONV-074, CONV-075 | conformance/conv/074/glossary.lo |
| CONV-076 | conformance/conv/076/reserved-words.lo |
| CONV-080 | conformance/conv/080/federated-registry.lo |

## Conformance categories

- **conv** — conventions & terminology (SPEC-00)
- **lex** — lexical structure (SPEC-01)
- **gram** — grammar (SPEC-02)
- **type** — type system (SPEC-03)
- **mem** — memory model (SPEC-04)
- **eff** — effects & concurrency (SPEC-05)
- **lir** — intermediate representation (SPEC-06)
- **back** — backends (SPEC-07)
- **ldb** — LinguaDB (SPEC-08)
- **tool** — toolchain (SPEC-09)
- **std** — standard library (SPEC-10)
- **sec** — security (SPEC-11)
- **comp** — compatibility & versioning (SPEC-12)
- **gov** — governance (SPEC-13)

A conforming implementation MUST pass 100% of tests for every prefix it declares as "pass" (CONV-040).