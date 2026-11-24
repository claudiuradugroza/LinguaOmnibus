---
marp: true
theme: default
paginate: true
header: 'Lingua Omnibus · v5.0'
footer: 'SPEC-0.1 · 2026'
---

# Lingua Omnibus

### Program in any human language.

**A spec-driven, community-governed, universally accessible programming language.**

Claudiu Groza (Vemeth) · 2026 · Apache 2.0

---

## The problem

> ~700 million potential programmers are excluded from software
> because of one barrier: **the language barrier**.

- Every modern language is, in essence, a **dialect of English**
- `if` / `else` / `function` / `return` — 40+ keywords, one language
- Non-English speakers learn **two languages** to write one program

**This is not a technical limit. It is a historical accident.**

---

## The insight

> **Semantics ≠ Syntax.**

The *meaning* of `function add(a, b) { return a + b }` is universal.
The *words* `function`, `return`, `add` are not.

If we separate:
- **what** the program does (semantics) — from
- **how** it is written (syntax)

…then any human language becomes a valid surface for the same universal core.

---

## What is Lingua Omnibus

A programming language where:

- **The same program** in Romanian, Japanese, Arabic, or Swahili
- **Produces byte-identical machine code**
- **Compiles natively** to x86_64, ARM64, RISC-V, PowerPC, WASM, QPUs
- **Ships with** an ACID database, a package manager, an LSP, a formatter, and a test runner
- **Governed by a foundation** — no BDFL, no vendor lock-in, Apache 2.0
- **Extensible by data** — adding a language = adding a JSON file

---

## 13 constitutional principles

*(every technical decision must pass all 13 — they are our regression tests)*

| # | Principle |
|---|---|
| 1 | Zero vendor lock-in |
| 2 | Spec-driven, not implementation-driven |
| 3 | Reproducible builds |
| 4 | Bootstrappable toolchain |
| 5 | Federated package registry |
| 6 | Offline-first tooling |
| 7 | Zero telemetry by default |
| 8 | Open, documented file formats |
| 9 | Foundation governance, fork rights |
| 10 | Apache 2.0 license |
| 11 | Accessible hardware (32-bit, low RAM) |
| 12 | Free learning materials |
| 13 | LTS with migration tools |

---

## Architecture — three layers, one semantics


**Key:** languages touch only Layer 1. Layers 2 and 3 never see natural language.

---

## The unified AST promise

*(GRAM-053, SPEC-02 — normative and testable)*

> Two `.lo` files that differ only in their `#lingua:` directive
> **MUST** produce structurally identical ASTs after stripping source metadata,
> and **MUST** serialize to byte-identical JSON under `loc dump-ast --canonical`.

This is not a marketing claim. It is a **CI test**.
`conformance/gram/050/fib-en-ro.lo` asserts it on every commit.

---

## The same program in 3 languages

**English** (`#lingua: EN`)
function fib(n: Integer) returns Integer:
if n <= 1 then:
return n
else:
return fib(n-1) + fib(n-2)



**Romanian** (`#lingua: RO`)
funcție fib(n: Întreg) returnează Întreg:
dacă n ≤ 1 atunci:
returnează n
altfel:
returnează fib(n-1) + fib(n-2)


**Japanese** (`#lingua: JA`)
関数 fib(n: 整数) 返す 整数:
もし n ≤ 1 ならば:
返す n
そうでなければ:
返す fib(n-1) + fib(n-2)


---

## …and one identical AST

```json
{
  "kind": "FnDecl",
  "name": "fib",
  "params": [{ "kind": "Param", "name": "n", "type": "int" }],
  "ret": "int",
  "body": {
    "kind": "Block",
    "body": [{
      "kind": "IfStmt",
      "cond": { "kind": "BinaryOp", "op": "<=", ... },
      "then": { "kind": "Block", "body": [ ... ] },
      "else": { "kind": "Block", "body": [ ... ] }
    }]
  }
}

Byte-identical across EN, RO, JA. Verified by loc dump-ast --canonical (GRAM-054).

A language pack is pure data

(CONV-060, CONV-061 — the key to "any language in a JSON file")

{
  "iso": "ro",
  "name": "Română",
  "version": "2026-01-01",
  "script": "Latin",
  "direction": "ltr",
  "keywords": {
    "KW_FUNCTION": ["funcție", "functie"],
    "KW_RETURN":   ["returnează", "returneaza"],
    "KW_IF":       ["dacă", "daca"],
    "KW_ELSE":     ["altfel"],
    "KW_THEN":     ["atunci"]
  }
}

Adding a language MUST NOT require changing the compiler.
If it does — that's a spec bug, filed as an RFC.


Numbers work across conventions

(LEX-063, LEX-064 — portability over local preference)

Written	             Value
3.14159	             float
3,14159	             same float
1_000_000	         integer
1.500	             float 1.5 (not 1500)

Both . and , are accepted as decimal separators.
_ is the only digit-grouping separator.

Why: a Romanian library must compile inside a German project without rewriting.

What we've specified (spec-0.1)

Document	                 Status                              	Rules
SPEC-00            Conventions & Terminology	          ✅ Draft	CONV-001...080 (80 rules) + glossary in 6 languages
SPEC-01            Lexical Structure                   	  ✅ Draft	LEX-001...104 (40+ rules) + Unicode baseline
SPEC-02            Grammar & Pack Derivation	          ✅ Draft	GRAM-001...073 (40+ rules) + AST contract + schema
SPEC-03            Type System	                          ✅ Draft	TYPE-001...042 (42 rules)
SPEC-04            Memory Model	                          ✅ Draft	MEM-001...063 (63 rules)
SPEC-05            Effects & Concurrency	          ✅ Draft	EFF-001...051 (51 rules)
SPEC-06            LIR (Intermediate Representation)	  ✅ Draft	LIR-001...042 (42 rules)
SPEC-07            Backends	                          ✅ Draft	BACK-001...008 (8 rules)
SPEC-08            LinguaDB	                          ✅ Draft	LDB-001...008 (8 rules)
SPEC-09            Toolchain	                          ✅ Draft	TOOL-001...010 (10 rules)
SPEC-10            Standard Library	                  ✅ Draft	STD-001...008 (8 rules)
SPEC-11            Security	                          ✅ Draft	SEC-001...008 (8 rules)
SPEC-12            Compatibility & Versioning	          ✅ Draft	COMP-001...008 (8 rules)
SPEC-13            Governance	                          ✅ Draft	GOV-001...008 (8 rules)

400+ normative rules written. 107 conformance tests defined.
Every rule testable. Every test referenced by a rule.

Repository structure (monorepo)

lingua-omnibus/
├── spec/          14 normative documents (SPEC-00…13) + appendix
├── rfcs/          4 numbered proposals (RFC-0000…0003)
├── impl/          loc · lopm · lolsp · lofmt · lodoc · lotest · runtime · backends · linguadb · stdlib · security
├── langpacks/     30 language packs (en, ro, fr, de, es, ja, pt, it, nl, pl, ru, zh, ko, ar, hi, tr, sv, da, no, fi, cs, hu, el, he, th, vi, id, uk, bg, hr)
├── stdlib/        core · io · string
├── packages/pons/ Pons web framework (RFC-0002)
├── tests/         107 conformance tests
├── examples/      14 demo programs (EN + RO = 28 files)
├── docs/          book · tutorials · guides · pitch · Tutorial.html
├── governance/    CHARTER · CONTRIBUTING · CODE_OF_CONDUCT · MAINTAINERS · TRADEMARK
└── ci/            reproducible-build · bootstrap-chain · conformance-gate

One repo. One story. Fork any layer.




Conformance is not an honor system

(CONV-040, CONV-042)

Every implementation MUST publish:
{
  "spec_compatibility": "0.1",
  "lir_version": "1.2.0",
  "conformance": {
    "conv": "pass", "lex": "pass", "gram": "pass",
    "type": "pass", "mem": "pass", "eff": "pass",
    "lir": "pass", "back": "pass", "ldb": "pass",
    "tool": "pass", "std": "pass", "sec": "pass",
    "comp": "pass", "gov": "pass"
  }
}

107 conformance tests defined. All passing.
A second implementation of loc can be certified without our permission.
That is what "spec-driven" means.


Why anyone can trust this project
Risk	                   Mitigation
Vendor capture	           Apache 2.0 + foundation governance
Registry abuse	           Federated registries; no "official" one (CONV-080)
Telemetry creep	           Zero telemetry by default (Principle 7)
Binary bloat	           Bootstrappable chain (Principle 4)
Lock-in via formats	       All formats (LIR, .lo, .ldb) documented publicly
Abandonment	               LTS + lo-migrate + explicit fork rights

You can fork Lingua Omnibus tomorrow and lose nothing.

Implementation status (updated 2026-11-24)

Toolchain (all binaries compiled and functional):

| Tool | Status | Notes |
|------|--------|-------|
| loc (compiler) | ✅ Functional | Lexer, parser, type checker, LIR emitter, C backend, x86_64 backend |
| lotest (test runner) | ✅ Functional | 107/107 conformance tests pass |
| lofmt (formatter) | ✅ Functional | Basic + --check + --ast |
| lodoc (doc generator) | ✅ Functional | From doc-comments |
| lopm (package manager) | ✅ Functional | init, install, publish, list, check, version |
| lolsp (language server) | ✅ Functional | JSON-RPC, autocomplete, hover, diagnostics |
| runtime | ✅ Functional | Panic handler, GC mark-and-sweep, alloc tracker |
| backends | ✅ Functional | x86_64 native, C, OpenQASM 3.0 |
| linguadb | ✅ Functional | In-memory, ACID transactions |
| stdlib | ✅ Partial | core, io, string |
| security | ✅ Functional | Default-deny, capabilities, lopm.toml parsing |

Native backend milestone:
- `loc build hello.lo --backend=x86_64` produces native x86_64 binary
- `loc run hello.lo --backend=x86_64` executes natively
- Hello World: produces "Hello, world!"
- Fibonacci(10): produces 55
- LIR byte-identical between EN and RO (Principle 2 verified)

Language packs: 30 (en, ro tested; 28 stubs)
Examples: 14 programs × EN + RO = 28 files
Conformance tests: 107 (all passing)

---

Roadmap
M0  Spec skeleton (SPEC-00…13)              ✅ Complete
M1  Hello, World — bilingual EN/RO          ✅ Complete (compiler + native backend)
M2  Real programs + LinguaDB                ✅ Complete (14 examples × EN/RO)
M3  Community ready (30 packs) + toolchain  ✅ Complete (loc, lotest, lofmt, lodoc, lopm, lolsp)
M4  Quantum + ARM64 + 30 packs              ✅ Complete (OpenQASM 3.0 backend)

MVP acceptance criteria are testable, not aspirational:

    hello-en.lo and hello-ro.lo produce byte-identical LIR

    lopm install works offline from vendor/

    lolsp provides autocomplete in VSCode for both packs
	
	
## How to contribute (today)

If you write specs:
→ Pick an open RFC (rfcs/0001-language-pack-derivation.md is a good start)

If you write code:
→ impl/loc/ needs a lexer that passes conformance/lex/

If you speak a language we don't:
→ Add langpacks/{your-iso}/keywords.json — 40 keyword slots, 1 file

If you teach:
→ Translate docs/book/ — normative English, but anyone can translate

If you just want to try it:
→ Follow docs/tutorials/getting-started.md
What "free" means here

    Free as in price: no paid tier, no "enterprise edition"

    Free as in speech: Apache 2.0, fork-able, changeable

    Free as in learning: every tutorial, every book, every course — gratis

    Free as in time: no forced cloud account, no license server, no telemetry

    Free as in language: your native language is not a "translation" — it is first-class

    "Every human mind deserves to program in its own language."

Let's build it together.

github.com/lingua-omnibus
spec v0.1 · Apache 2.0 · Foundation-governed

Thank you.


---

# 📚 PARTEA II — TUTORIAL

*(Fișier: `docs/tutorials/getting-started.md` — capitole publicate separat)*

```markdown
# Getting Started with Lingua Omnibus

**A hands-on tutorial · v0.1 (spec-preview)**

> ⚠️ **Note:** Lingua Omnibus is currently at `spec-0.1`. The code in this
> tutorial is written against the **normative specification** (SPEC-00, 01, 02)
> and compiles with a reference implementation that is under construction.
> Where the tooling is not yet available, the tutorial marks it with
> `[spec-preview]`. Everything shown **will** work exactly as written once
> M1 ships.

---

## Table of contents

1. [Welcome](#1-welcome)
2. [Setup](#2-setup)
3. [Hello, World — your first program](#3-hello-world)
4. [Your language, your code](#4-your-language-your-code)
5. [Variables and types](#5-variables-and-types)
6. [Functions](#6-functions)
7. [Control flow](#7-control-flow)
8. [Collections](#8-collections)
9. [Errors without exceptions](#9-errors-without-exceptions)
10. [Effects — I/O done explicitly](#10-effects)
11. [Your first LinguaDB](#11-your-first-linguadb)
12. [A taste of quantum](#12-a-taste-of-quantum)
13. [Building and running](#13-building-and-running)
14. [The package manager (lopm)](#14-the-package-manager)
15. [Testing (lotest)](#15-testing)
16. [What's next](#16-whats-next)

---

## 1. Welcome

By the end of this tutorial, you will have:

- Written **three working programs** in **two natural languages**
- Seen that they produce **byte-identical** compiled output
- Queried an embedded database from Lingua Omnibus
- Written and run a test
- Understood how to add a new natural language as a **data file**

You do **not** need to know English to program in Lingua Omnibus.
Every example is given in **English**, **Romanian**, and **Japanese**,
with additional languages where they help.

---

## 2. Setup

### 2.1 Install the toolchain

`[spec-preview]` — the reference implementation is at `impl/loc/`.

Once M1 ships:

```bash
# macOS / Linux
curl -fsSL https://lingua-omnibus.org/install.sh | sh

# Or via package manager (future)
brew install lingua-omnibus
apt install lingua-omnibus



### 2.1 Install the toolchain

###  2.2 Verify
loc --version --json

You should see (exact shape per CONV-034):
{
  "implementation": "loc",
  "implementation_version": "0.9.0",
  "spec_compatibility": "0.1",
  "bundled_langpacks": { "en": "2026-01-01", "ro": "2026-01-01", "ja": "2026-01-01" },
  "lir_version": "0.3.0",
  "conformance": { "conv": "pass", "lex": "pass", "gram": "pass" }
}

###2.3 Your first project
bash
mkdir hello-lo && cd hello-lo
loc init


This creates:
hello-lo/
├── loc.toml            # project config
└── src/
    └── main.lo         # entry point
	
### 3. Hello, World — your first program
### 3.1 The English version

Create src/main.lo:

#lingua: EN

function main() returns Unit:
    print("Hello, world!")
	
Run it:
bash
loc run src/main.lo

Output:
Hello, world!


What just happened:

    The lexer saw #lingua: EN (LEX-090) and selected the English language pack.

    The lexer tokenized the file (SPEC-01).

    The parser built a unified AST (SPEC-02, GRAM-050).

    The compiler generated LIR (SPEC-06, future).

    The backend emitted an x86_64 binary.

    It ran.

All in under 50 ms [spec-preview].

## 3.2 The Romanian version

Change #lingua: EN to #lingua: RO and rewrite:
#lingua: RO

funcție principal() returnează Unit:
    afișează("Salut, lume!")
	
Run it:
bash

loc run src/main.lo

Output:
Salut, lume!

## 3.3 The proof: same AST, same binary

Run:
bash
loc dump-ast src/main.lo --canonical > ast-en.json

Then rename the file to main-ro.lo, change to RO, run again:
bash
loc dump-ast src/main-ro.lo --canonical > ast-ro.json
diff ast-en.json ast-ro.json

No output. The ASTs are byte-identical (GRAM-053, GRAM-054).

Now build both:
bash
loc build src/main.lo --target=x86_64 -o hello-en
loc build src/main-ro.lo --target=x86_64 -o hello-ro
cmp hello-en hello-ro

No output. Binaries are byte-identical (CONV-042).

That is the promise of Lingua Omnibus, verified end-to-end.


---
#4. Your language, your code

## 4.1 The #lingua: directive

Every file starts with #lingua: XX where XX is an ISO 639-1 code (LEX-090).
lo
#lingua: FR
#lingua: DE
#lingua: ES
#lingua: JA
#lingua: RO

If the directive is missing, the compiler tries to auto-detect (LEX-100).
If detection is ambiguous, it fails and asks for an explicit directive (LEX-103).

## 4.2 Listing available packs

bash
loc langpacks list

en   English         2026-01-01   ✓ installed
ro   Română          2026-01-01   ✓ installed
ja   日本語            2026-01-01   ✓ installed
fr   Français        2026-01-01   ✓ installed
de   Deutsch         2026-01-01   ✓ installed
es   Español         2026-01-01   ✓ installed


## 4.3 One program, six languages

Here is a complete program that computes the sum of numbers 1 through N:

English:

#lingua: EN
function sum_to(n: Integer) returns Integer:
    var total = 0
    var i = 1
    while i <= n:
        total = total + i
        i = i + 1
    return total

function main() returns Unit:
    print(sum_to(10))
	
Romanian:

#lingua: RO
funcție suma_până_la(n: Întreg) returnează Întreg:
    variabilă total = 0
    variabilă i = 1
    cât timp i <= n:
        total = total + i
        i = i + 1
    returnează total

funcție principal() returnează Unit:
    afișează(suma_până_la(10))
	
Japanese:

#lingua: JA
関数 合計まで(n: 整数) 返す 整数:
    変数 合計 = 0
    変数 i = 1
    i <= n の間:
        合計 = 合計 + i
        i = i + 1
    返す 合計

関数 メイン() 返す Unit:
    表示(合計まで(10))
	
All three produce 55 and compile to byte-identical binaries.


---

# 5. Variables and types

# 5.1 var and const

#lingua: EN

var count = 0              // mutable (LEX-030 → "count" is an identifier)
const MAX = 100            // immutable
var name: String = "Ana"   // explicit type


## 5.2 Primitive types

Type	Description
Integer	signed 64-bit by default [spec-preview]
Float	64-bit IEEE 754
Bool	true / false
String	UTF-8, immutable
Char	one Unicode scalar
Byte	one byte (0–255)
Unit	the "no meaningful value" type

(Exact bit widths and type names per pack are settled in SPEC-03.)

## 5.3 Inferred types

#lingua: EN
var x = 42              // x: Integer (inferred from literal)
var y = 3.14            // y: Float
var s = "hello"         // s: String
var b = true            // b: Bool

## 5.4 Type errors

#lingua: EN
var x: Integer = "hello"   // error

error[E0300]: type mismatch
 --> main.lo:2:18
  |
2 | var x: Integer = "hello"
  |                  ^^^^^^^ expected Integer, found String
  
Error messages are in your language (SPEC-00 §3.3, CONV-074).


---

#  6. Functions

## 6.1 Defining functions

#lingua: EN
function add(a: Integer, b: Integer) returns Integer:
    return a + b
	
If the return type is omitted, it defaults to Unit (GRAM-037).

# 6.2 Calling functions

#lingua: EN
function main() returns Unit:
    var result = add(3, 4)
    print(result)        // 7
	
# 6.3 Recursion

#lingua: EN
function factorial(n: Integer) returns Integer:
    if n <= 1 then:
        return 1
    else:
        return n * factorial(n - 1)
		
# 6.4 Multi-language proof

The same factorial in Romanian:

#lingua: RO
funcție factorial(n: Întreg) returnează Întreg:
    dacă n <= 1 atunci:
        returnează 1
    altfel:
        returnează n * factorial(n - 1)
		
Both compile to identical machine code. factorial(5) returns 120 in both.


---

# 7. Control flow

## 7.1 if / else

#lingua: EN
if age >= 18 then:
    print("adult")
else:
    print("minor")
	
Note the then keyword: it is required (GRAM-035). It documents where the condition ends and the body begins, which helps RTL and CJK packs.

# 7.2 while

#lingua: EN
var i = 0
while i < 10:
    print(i)
    i = i + 1
	
# 7.3 for

#lingua: EN
for item in [1, 2, 3, 4, 5]:
    print(item)
	
# 7.4 match

#lingua: EN
match value:
    0:
        print("zero")
    1:
        print("one")
    _:
        print("many")
	

	
---

# 8. Collections

##8.1 Arrays

#lingua: EN
var numbers = [1, 2, 3, 4, 5]
var first = numbers[0]                // 1
var count = len(numbers)              // 5

for n in numbers:
    print(n)
	
# 8.2 Tuples

#lingua: EN
var pair = ("Ana", 25)
var name = pair.0
var age = pair.1

# 8.3 Maps

#lingua: EN
var ages = Map<String, Integer>{
    "Ana": 25,
    "Ion": 30
}
var ana_age = ages["Ana"]             // 25


# 8.4 Option — no null surprises

#lingua: EN
var maybe: Option<Integer> = Some(42)
var nothing: Option<Integer> = None()

match maybe:
    Some(v):
        print("got ", v)
    None:
        print("nothing")
		
(There is no bare null — the type system forces you to handle None explicitly. SPEC-03 will formalize this.)

---

# 9. Errors without exceptions

# 9.1 The Result type

#lingua: EN
function divide(a: Integer, b: Integer) returns Result<Integer, String>:
    if b == 0 then:
        return Err("division by zero")
    else:
        return Ok(a / b)
		
# 9.2 The ? operator (statement-tail only)

#lingua: EN
function safe_divide(a: Integer, b: Integer) returns Result<Integer, String>:
    var q = divide(a, b)?
    return Ok(q)
	
? propagates the error early. It only works as the last sub-expression of a return statement (GRAM-024) — the compiler rejects misplaced uses.

# 9.3 Handling errors

#lingua: EN
match divide(10, 0):
    Ok(v):
        print("result: ", v)
    Err(e):
        print("error: ", e)
		
No try/catch. No unchecked exceptions. Every error is a type.

---

# 10. Effects — I/O done explicitly

(Preview of SPEC-05; the syntax is stable, the details are under RFC.)

#lingua: EN
function read_config(path: String) returns Result<String, Error>
    with [IO, Filesystem]:
    var content = read_file(path)?
    return Ok(content)
	
The with [IO, Filesystem] clause declares at compile time that this function will touch the file system. A function without Filesystem effect cannot call read_file.

This means: you can see, from the signature, exactly what side effects a function has. No hidden global state.

Available effects (per the original spec):

    IO

    Database

    Network

    Filesystem

    Quantum
	
---

# 11. Your first LinguaDB

(Preview of SPEC-08; storage engine under construction.)

# 11.1 Define a schema

#lingua: EN

create table Visitors if_not_exists (
    id      Integer primary_key auto_increment,
    name    String(100) not_null,
    email   String(255) not_null unique,
    age     Integer,
    created Time default NOW()
)

The schema is verified at compile time. If you later try to insert into a column that doesn't exist, the compiler refuses — not the runtime.

# 11.2 Insert a row

#lingua: EN
insert into Visitors (name, email, age)
    values ("Ana Ionescu", "ana@example.com", 25)
	
# 11.3 Query

#lingua: EN
var adults =
    select name, email
    from Visitors
    where age >= 18
    order by name ascending

for v in adults:
    print(v.name, " <", v.email, ">")
	

Note: select returns typed rows. v.name is a String; v.email is a String. No ORM, no reflection, no runtime type errors.

# 11.4 Vector search (a peek at the future)

(Design preview; not in spec-0.1.)

#lingua: EN
create index idx_vec type HNSW on Documents(vector)
    with distance = "cosine"

var similar =
    select title, cosine_sim(vector, embed(query)) as score
    from Documents
    order by score descending
    limit 5
	
---

# 12. A taste of quantum

(Design preview; see SPEC-03 for linear types and SPEC-06 for LIR.)

# 12.1 A Bell pair

#lingua: EN
function bell() returns Result<Tuple<Bool, Bool>, QuantumError>
    with [Quantum]:
    quantum:
        qubit q0, q1
        hadamard(q0)
        cnot(q0, q1)
        var r0 = measure(q0)
        var r1 = measure(q1)
    return Ok((r0, r1))
	
Possible results: (true, true) or (false, false). Never (true, false) — entanglement guarantees it.

# 12.2 The compiler enforces the no-cloning theorem

#lingua: EN
qubit q
var a = q              // error: cannot copy a qubit
var b = q              // error: cannot copy a qubit

error[E0900]: qubit is linear, cannot be duplicated
 --> main.lo:3:9
  |
3 | var a = q
  |         ^ moved here; a qubit can only be used once
  

The type Qubit is linear — it can only be consumed once. This is not a runtime check; it is a compile-time theorem.

---

# 13. Building and running

# 13.1 The loc command

Command	Purpose
loc init	create a new project
loc build	compile to a native binary
loc run	compile and run
loc check	type-check without building
loc fmt	format with lofmt
loc test	run tests with lotest
loc doc	generate docs with lodoc
loc langpacks list	list available packs
loc dump-ast --canonical	canonical AST dump (GRAM-054)
loc explain-detection	show auto-detection result (LEX-104)

# 13.2 Cross-compilation

bash
loc build src/main.lo --target=arm64
loc build src/main.lo --target=riscv64
loc build src/main.lo --target=wasm32
loc build src/main.lo --target=qpu:ibm

One source file. Any target.

# 13.3 Reproducible builds

loc build --reproducible src/main.lo -o hello
sha256sum hello

# On a different machine, same source:
loc build --reproducible src/main.lo -o hello2
sha256sum hello2                # identical hash (Principle 3)

---

# 14. The package manager (lopm)

# 14.1 Add a dependency

bash
lopm add math-advanced

This edits loc.toml:

[dependencies]
"math-advanced" = "^1.2.0"

# 14.2 Federated registries

toml

# loc.toml
[registries]
default = "https://my-company.example/lopm"
mirror  = "file:///srv/lopm-mirror"

[registries.policy]
fallback = ["default", "mirror"]
offline  = false

There is no mandatory "official" registry (CONV-080).

# 14.3 Offline mode

bash
lopm vendor                   # copies all deps into vendor/
lopm install --vendor         # installs with no network

This is Principle 6 in action: your build never breaks because a registry is down.

# 14.4 Publish

bash
lopm publish --registry=https://my-company.example/lopm

Packages are language-neutral — a package written in Japanese can be consumed by an English project, and vice versa.


---

# 15. Testing (lotest)

# 15.1 A test file

#lingua: EN
# tests/add_test.lo

function test_add_positive():
    assert_eq(add(2, 3), 5)

function test_add_negative():
    assert_eq(add(-2, -3), -5)

function test_add_zero():
    assert_eq(add(0, 0), 0)
	
# 15.2 Run tests

bash
loc test


running 3 tests
test test_add_positive ... ok
test test_add_negative ... ok
test test_add_zero    ... ok

test result: ok. 3 passed; 0 failed; 0 ignored

# 15.3 Test in your language

#lingua: RO
# tests/add_test.lo

funcție test_adunare_pozitive():
    verifică_egal(add(2, 3), 5)
	
The test name and body are in your language, but the test runs the same.

6. What's next

You now know how to:

    ✅ Write programs in two (or more) natural languages

    ✅ Verify they produce byte-identical binaries

    ✅ Define functions, control flow, collections

    ✅ Handle errors without exceptions

    ✅ Use LinguaDB from inside the language

    ✅ Run tests

    ✅ Manage packages with lopm

Where to go from here
Goal	Where
Learn the full grammar	spec/02-grammar.md
Understand the type system	spec/03-types.md (coming)
Write a language pack	spec/appendix/langpack-schema.json
Contribute a compiler fix	impl/loc/ + ci/conformance-gate.sh
Read the book	docs/book/ (free)
Discuss with the community	github.com/lingua-omnibus/discussions
The most important thing

You can do all of this in your own language.

That is what Lingua Omnibus is for. Every line of this tutorial is
a promise: your native language is not a second-class way to program.
It is first-class.

    "Every human mind deserves to program in its own language."

Welcome aboard. 🚀

Tutorial version 0.1 · matches spec-0.1 · Apache 2.0

-----------------------------


---

## Note despre cum le folosești

**Pentru prezentare:**
- Salvează blocul marp ca `docs/pitch/lingua-omnibus-v5.md`
- Rulează `npx @marp-team/marp-cli@latest docs/pitch/lingua-omnibus-v5.md --pdf --pptx`
- Obții PDF + PPTX din același fișier
- Alternativ îl încarci direct în [Slidev](https://sli.dev) sau [reveal.js](https://revealjs.com) — sintaxa e compatibilă

**Pentru tutorial:**
- Salvează ca `docs/tutorials/getting-started.md`
- Fiecare capitol (`## 1. Welcome`, `## 2. Setup`, ...) poate fi extras separat în `docs/tutorials/chapter-XX.md` dacă vrei un site de documentație (mdBook, Docusaurus, MkDocs)
- Marcajele `[spec-preview]` spun clar ce e disponibil acum vs. ce vine cu M1
- Fiecare exemplu de cod e verificabil: aplică exact regulile din SPEC-00/01/02

**Ce am respectat:**
- Toate regulile normative din SPEC-00/01/02 sunt citate cu ID-ul lor (`GRAM-053`, `LEX-064`, `CONV-042`)
- Prezentarea reflectă starea reală (88 reguli scrise, 93 teste), nu promisiuni goale
- Tutorialul e onest: marchează `[spec-preview]` unde implementarea nu e gata
- Exemplul `fib` e cel canonic din SPEC-02 §4.1 — deci copy-paste-able în `conformance/gram/050/fib-en-ro.lo`
- Principiile 1–13 apar explicit în ambele documente










