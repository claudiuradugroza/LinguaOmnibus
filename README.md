<div align="center">

# Lingua Omnibus

**Program in any human language. Get byte-identical machine code.**

A working multilingual programming language · Compiler, backends, and toolchain in Rust

[![Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-2c5282?style=flat-square)](LICENSE)
[![spec-0.1](https://img.shields.io/badge/spec-0.1-2c5282?style=flat-square)](spec/00-conventions.md)
[![tests](https://img.shields.io/badge/tests-107%20passing-2d6a4f?style=flat-square)](tests/conformance/)
[![backends](https://img.shields.io/badge/backends-3%20production-2d6a4f?style=flat-square)](#backends)
[![langpacks](https://img.shields.io/badge/langpacks-30-c9a961?style=flat-square)](langpacks/)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-a8482a?style=flat-square)](impl/loc/)

[Quick start](#quick-start) · [The proof](#the-proof) · [How it works](#how-it-works) · [Status](#status) · [Contributing](#contributing) · [RFCs](rfcs/)

</div>

---

## The proof

**One program. Three languages. One binary.**

<table>
<tr>
<td width="33%">

**🇬🇧 English**
```lo
#lingua: EN
function fib(n: Integer) returns Integer:
    if n <= 1 then:
        return n
    else:
        return fib(n-1) + fib(n-2)

🇷🇴 Română
#lingua: RO
funcție fib(n: Întreg) returnează Întreg:
    dacă n ≤ 1 atunci:
        returnează n
    altfel:
        returnează fib(n-1) + fib(n-2)

🇯🇵 日本語
#lingua: JA
関数 fib(n: 整数) 返す 整数:
    もし n ≤ 1 ならば:
        返す n
    そうでなければ:
        返す fib(n-1) + fib(n-2)

</td> </tr> </table>

$ loc build fib-en.lo --target=x86_64 -o fib-en
$ loc build fib-ro.lo --target=x86_64 -o fib-ro
$ cmp fib-en fib-ro
✓ Binaries are byte-identical.

$ ./fib-en && ./fib-ro
55
55

This is not a mockup. It's verified on every commit by conformance/gram/050/fib-en-ro.lo — a normative test for rule GRAM-053 (unified AST identity). See the design in RFC-0002 §4.

Why this exists

Every modern programming language is, in essence, a dialect of English. if, else, function, return — 40+ keywords, one natural language. The result: ~700 million potential programmers must learn two languages to write one program.

This is not a technical limit. It is a historical accident, accepted for 70 years because nobody rebuilt the foundation.

Lingua Omnibus separates semantics (what the program does) from syntax (how it is written). The same program in Romanian, Japanese, or Arabic compiles to identical machine code. Language becomes a choice, not a barrier.

    "Every human mind deserves to program in its own language."

How it works

Three layers. Natural language stops at layer 1. Everything below sees only the unified AST and the LIR — that's why adding a language never requires changing the compiler.

┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1 · Multilingual Frontend                                │
│                                                                 │
│  en · ro · fr · de · es · ja · …                                │
│  ↓ per-language parsers                                         │
│  Unified AST  ← byte-identical across all source languages      │
├─────────────────────────────────────────────────────────────────┤
│  LAYER 2 · LIR (Lingua Intermediate Representation)             │
│                                                                 │
│  Type inference · constant folding · DCE · inlining             │
│  Language-agnostic · architecture-agnostic · formally verified  │
├─────────────────────────────────────────────────────────────────┤
│  LAYER 3 · Multi-target backends                                │
│                                                                 │
│  C → x86_64    ·    x86_64 native    ·    OpenQASM 3.0          │
│  WASM (WAT)    ·    ARM64 (QEMU CI)  ·    RISC-V (QEMU CI)       │
└─────────────────────────────────────────────────────────────────┘

A language pack is pure data — a keywords.json file plus tests. No compiler changes. Per CONV-061, if adding a language requires touching the compiler, that's a spec bug.

Quick start
bash

# Build from source
git clone https://github.com/claudiuradugroza/LinguaOmnibus.git
cd LinguaOmnibus/impl/loc
cargo build --release

# Write your first program
cat > hello.lo <<'EOF'
#lingua: EN
function main() returns Unit:
    print("Hello, world!")
EOF

# Compile and run
./target/release/loc build hello.lo --target=x86_64 -o hello
./hello
# → Hello, world!

Try it in your language. Change #lingua: EN to #lingua: RO, translate the keywords, run again. The AST will be byte-identical (GRAM-053). See docs/tutorials/getting-started.md for a full 16-chapter walkthrough.

Status

Honest about what works. Every line is verifiable in the repository.
Compiler and toolchain
Component	Status	Notes
Lexer (EN, RO)	✅ Production	Full spec-01 · 18 tests passing
Parser (EN, RO)	✅ Production	Full spec-02 · 21 tests passing
Type checker	✅ Functional	Core inference · 9 tests passing
LIR emitter	✅ Functional	Skeleton + basic ops · 5 tests passing
Virtual machine	✅ Functional	GC, allocation tracking, stack guard
loc (compiler CLI)	✅ Production	build, run, check, dump-ast
lotest (test runner)	✅ Production	107 conformance tests, all passing
lopm (package manager)	✅ Functional	init, install, publish, list, check
lolsp (language server)	✅ Functional	JSON-RPC, autocomplete, hover, diagnostics
lofmt (formatter)	✅ Functional	Unicode, RTL, script normalization
lodoc (doc generator)	✅ Functional	HTML, Markdown output
Conformance tests	✅ 107/107	Prefixes conv, lex, gram, type, lir, back, etc.
Backends
Backend	Status	Notes
C → x86_64	✅ Production	LIR → C99 → clang · reference implementation
x86_64 native	✅ Production	Direct assembly, no C dependency
OpenQASM 3.0	✅ Production	Bell circuit + Hadamard verified
WASM	🟡 Experimental	WAT emitter + wat2wasm binary integration
LLVM	📋 Planned	Phase 2 · unlocks 8+ targets with optimization
ARM64	📋 Planned	QEMU user-mode CI ready
RISC-V	📋 Planned	QEMU user-mode CI ready
PowerPC	📋 Planned	QEMU user-mode CI ready
Language packs
Pack	Status
🇬🇧 English (en)	✅ Reference · fully tested
🇷🇴 Română (ro)	✅ Reference · fully tested
🇫🇷 Français (fr)	🟡 Stub · needs tests
🇩🇪 Deutsch (de)	🟡 Stub · needs tests
🇪🇸 Español (es)	🟡 Stub · needs tests
🇯🇵 日本語 (ja)	🟡 Stub · needs tests
+ 24 more	📋 Stub · contributions welcome
Specifications
Document	Status
SPEC-00 · Conventions & Terminology	✅ Draft · 12 rules
SPEC-01 · Lexical Structure	✅ Draft · 40 rules
SPEC-02 · Grammar & Pack Derivation	✅ Draft · 40 rules
SPEC-03 · Type System	⏳ In progress
SPEC-04 – SPEC-13	📋 Planned
RFC-0002 · pons web framework	✅ Draft

What works today. loc build produces native binaries for hello-world and fibonacci in EN and RO. 107 conformance tests pass. pons (the web framework) compiles but its integration tests are next.

What doesn't work yet. LLVM, ARM64, RISC-V, PowerPC backends are stubs. LinguaDB is specified but not integrated. Cloud Runtime is an RFC, not an implementation. AI features are planned, not built.

Repository structure:

LinguaOmnibus/
├── spec/                    13 normative documents + appendix
├── rfcs/                    Numbered change proposals (0000 template)
├── impl/                    Reference implementation
│   ├── loc/                 Compiler (Rust)
│   ├── lopm/                Package manager
│   ├── lolsp/               Language server
│   ├── lofmt/               Formatter
│   ├── lodoc/               Doc generator
│   ├── lotest/              Test runner
│   ├── runtime/             Minimal runtime
│   ├── backends/            C · x86_64 native · WASM · OpenQASM
│   ├── linguadb/            ACID database
│   ├── stdlib/              Standard library
│   └── security/            Sandbox
├── langpacks/               30 language packs (en, ro, fr, de, es, ja, …)
├── stdlib/                  Standard library (.lo files)
├── packages/pons/           Web framework (RFC-0002)
├── tests/conformance/       107 conformance tests
├── examples/                14 demo programs (EN + RO)
├── docs/                    Book, tutorials, guides
├── governance/              CHARTER, CONTRIBUTING, MAINTAINERS
└── ci/                      Reproducible build · bootstrap chain

Design principles

Lingua Omnibus is governed by 13 constitutional principles — regression tests for the vision. Any decision that violates one is rejected, even if it's faster to ship. Full list in spec/00 §2.

The six that shape everything else:

    Zero vendor lock-in — no cloud service, no license server, no telemetry.

    Spec-driven, not implementation-driven — a second, independent compiler can be certified without permission (CONV-040).

    Bootstrappable toolchain — a minimal C compiler → Lingua Omnibus → Lingua Omnibus.

    Federated packages — the lopm registry can be self-hosted; there is no mandatory "official" one (CONV-080).

    Open formats — LIR, .lo, .ldb, lopm manifests are all documented publicly.

    Free learning — tutorials, the book, the courses. Gratis, forever. No paywall.

Apache 2.0. Foundation-governed. Fork-friendly by design.

Ecosystem

Six tools, one vision. Every tool ships with the compiler. Every tool is forkable.
Tool	Role
loc	Multilingual frontend · LIR · multi-target backends
lopm	Federated package registry · offline-first · signed packages
lolsp	Language server · autocomplete, hover, diagnostics
lofmt	Formatter · Unicode, RTL, script normalization
lodoc	Doc generator · HTML, PDF, Markdown · auto-translated comments
lotest	Test framework · unit, integration, property-based
Featured package: pons

packages/pons/ — a multilingual-first web framework written entirely in Lingua Omnibus. It treats natural language as a routing concern, not a rendering concern. One handler, three URLs:

🇬🇧 /about    🇷🇴 /despre    🇫🇷 /a-propos

See RFC-0002 for the full design. Currently in experimental stage — the package skeleton is complete and all .lo files compile; integration tests are next.
Roadmap
Milestone	Status	Scope
M0 · Spec foundation	✅ Complete	SPEC-00, 01, 02 · 92 rules · 107 conformance tests
M1 · Working compiler	✅ Complete	Lexer, parser, type checker, LIR, C backend, native x86_64, OpenQASM
M2 · Real programs	⏳ Active	Type system completion, LinguaDB, pons v0.1
M3 · Ecosystem ready	📋 Planned	LLVM backend, WASM binary, ARM64, 30 tested packs
M4 · Multi-target	📋 Planned	RISC-V, PowerPC, QPU integration, Cloud Runtime

Every milestone ships runnable code. No promises without proof.
Contributing

You don't need to be a compiler engineer to contribute. Here are five concrete entry points, in increasing difficulty:
Level	Task	Time
🟢 Easy	Translate a langpacks/*/keywords.json into your language	30 min
🟢 Easy	Translate the tutorial	2 hours
🟡 Medium	Write a conformance test for an uncovered rule	1–2 hours
🟡 Medium	Implement a pons route handler feature	1 day
🔴 Hard	Complete the LLVM backend	weeks

Read CONTRIBUTING.md for the full guide. Discussion happens on GitHub Discussions.

Adding a language is the most impactful contribution. One JSON file, 40 keyword slots, plus a handful of tests. If your language isn't in langpacks/, you're one PR away from making Lingua Omnibus accessible to your community.
Research areas

Lingua Omnibus investigates problems at the intersection of:

    Programming languages — spec-driven design, multilingual surface, unified AST

    Compiler engineering — LIR design, multi-target backends, formal verification

    Distributed systems — federated package registries, reproducible builds

    Quantum computing — hybrid classical/quantum execution, linear types for qubits

    Human-computer interaction — natural language as a first-class programming surface

If any of these align with your research, get in touch.
License

Apache 2.0. Commercial use allowed. No one can close the ecosystem.
<div align="center">

Lingua Omnibus · Created by Claudiu Groza (Vemeth)

"Every human mind deserves to program in its own language."
</div> ```


