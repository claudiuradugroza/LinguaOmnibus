# RFC-0002: pons — A Multilingual-First Web Framework for Lingua Omnibus

Status: Draft
Author: Claudiu Groza (Vemeth)
Created: 2026-11-23
Last updated: 2026-11-23
Tracking issue: (to be opened)
Discussion: (to be opened on GitHub Discussions)
Target spec version: spec-0.1 (no spec changes required)
Depends on: SPEC-00, SPEC-01, SPEC-02, SPEC-05 (planned), SPEC-08 (planned), SPEC-09 (planned), SPEC-10 (planned)
Supersedes: none
Superseded by: none

## 1. Summary

This RFC proposes pons, a multilingual-first web framework for Lingua Omnibus, written in Lingua Omnibus itself. pons treats natural language as a first-class concern not only for the programmer (as the language already does) but for the end user of the applications built with it — routes, validation messages, error pages, and templates all adapt to the user's language without duplicating application logic.

pons is not part of the language specification. It is a normal package, distributed via lopm, forkable, replaceable, and subject to no special privilege. It exists to prove that Lingua Omnibus is expressive enough to host its own ecosystem.

## 2. Motivation

### 2.1 The problem with existing frameworks

Every mainstream web framework treats internationalization (i18n) as a plugin, not a premise. The result is that multilingual applications are structurally second-class. Programmers write scattered language checks or accept English-only routes.

### 2.2 The Lingua Omnibus opportunity

Lingua Omnibus was created to make the programmer's natural language first-class (SPEC-00 section 1, CONV-074). pons extends that promise to the end user's natural language.

### 2.3 Why now

Three preconditions are now met: SPEC-00, 01, 02 are stable drafts; the unified AST contract (GRAM-053) guarantees byte-identical LIR; the 13 constitutional principles already commit the project to interop and federation.

## 3. Guide-level explanation

### 3.1 Installation

```
lopm add pons
```

pons is a normal package. It has a loc.toml, a version, a SemVer. It is distributed through any lopm registry.

### 3.2 The canonical example

```lo
#lingua: EN
import pons

function index(req: Request) returns Response with [IO]:
    return pons.html("<h1>Hello, world!</h1>")

function main() returns Result<Unit, AppError> with [IO, Network]:
    var app = pons.new_app()
    app.get("/", index)
    return app.listen(8080)
```

```lo
#lingua: RO
importă pons

funcție index(cerere: Cerere) returnează Răspuns cu [IO]:
    returnează pons.html("<h1>Salut, lume!</h1>")

funcție principal() returnează Result<Unit, AppError> cu [IO, Network]:
    variabilă app = pons.aplicație_nouă()
    app.obține("/", index)
    returnează app.ascultă(8080)
```

Both files compile to byte-identical binaries (via CONV-042).

### 3.3 Multilingual routes

Routes can be declared with per-language URLs:

```lo
@app.get(route: { "en": "/about", "ro": "/despre", "fr": "/a-propos" })
function about(req: Request) returns Response with [IO]:
    return pons.render("about.html", {
        title: pons.t("about.title"),
        body:  pons.t("about.body")
    })
```

### 3.4 LinguaDB integration — no ORM

```lo
struct Visitor:
    id:      Integer primary_key auto_increment
    name:    String(100) not_null
    email:   String(255) not_null unique
    age:     Integer
    created: Time default NOW()

@app.post("/visitors")
function create_visitor(req: Request) returns Response with [Database, IO]:
    var body = req.json()
    var validated = validate(body)
    var visitor = db.insert(Visitor, validated)
    return pons.json(201, visitor)
```

### 3.5 Validation with end-user messages

```lo
function validate(input: VisitorInput) returns Result<VisitorInput, ValidationError>:
    if not input.email.contains("@"):
        return Err(ValidationError(field: "email", code: "email.invalid"))
    if input.age < 0 or input.age > 150:
        return Err(ValidationError(field: "age", code: "age.out_of_range"))
    return Ok(input)
```

### 3.6 Effects visible in the signature

```lo
@app.post("/process-payment")
function process_payment(req: Request) returns Response with [IO, Database, Network]:
    var order  = db.find(Order, req.path.id)
    var result = payment_gateway.charge(order)
    db.update(order, status: "paid")
    return pons.json(200, { status: "ok" })
```

## 4. Reference-level explanation

### 4.1 Where pons lives

Phase 1 (spec-0.1 to M2): inside the monorepo at packages/pons/.
Phase 2 (post-M3): migrates to its own repository.

### 4.2 Package layout

```
packages/pons/
├── loc.toml
├── README.md
├── src/
│   ├── main.lo
│   ├── app.lo
│   ├── router.lo
│   ├── request.lo
│   ├── response.lo
│   ├── middleware.lo
│   ├── handlers.lo
│   ├── validate.lo
│   ├── i18n.lo
│   ├── db.lo
│   └── html.lo
├── locales/
│   ├── en.json
│   ├── ro.json
│   ├── fr.json
│   ├── de.json
│   ├── es.json
│   └── ja.json
├── tests/
└── examples/
```

### 4.3 Public API (v0.1 minimum)

Types: App, Request, Response, ValidationError
Functions: new_app, listen, get, post, put, delete, html, json, t, set_lang, db.insert, db.find, db.update

### 4.4 Route resolution algorithm

1. Parse Accept-Language header
2. Check query param ?lang=ro
3. Check cookie pons_lang
4. Fall back to default language
5. If no route matches, return 404
6. If language not supported, return 406

### 4.5 Locale catalog format

JSON files with message keys and translations. Missing translations fall back to default language.

### 4.6 Relationship to LinguaDB

pons.db is a thin wrapper over LinguaDB (SPEC-08). No ORM layer.

### 4.7 What pons deliberately does NOT include

No template engine, no authentication, no static file serving, no WebSocket, no migrations, no admin panel. Minimal coherent core.

## 5. Drawbacks

1. Coupling to LinguaDB — thin wrapper mitigates
2. Bootstrapping risk — pons depends on impl/loc
3. Scope creep — v0.1 API freeze
4. Fragmentation — federated packages allow alternatives
5. No prior art for multilingual routes

## 6. Rationale and alternatives

- Alternative A (Rust): Rejected — destroys self-hosting proof
- Alternative B (part of spec): Rejected — violates Principle 2
- Alternative C (Python/TS transpiled): Rejected — violates Principle 1
- Alternative D (existing framework): Rejected — no framework treats i18n as first-class
- Alternative E (existing web server library): Considered, deferred

## 7. Prior art

Rails, Phoenix, Axum, Django, FastAPI, Laravel, Spring Boot, Next.js, Hono — all treat i18n as afterthought. pons is genuinely novel in multilingual routes.

## 8. Unresolved questions

1. Route declaration syntax — verbose, alternatives proposed
2. Template language — deferred to v0.4
3. Locale detection precedence — needs user feedback
4. Missing-language behavior — redirect vs render default
5. Session management — deferred to v0.5
6. Error handling at framework level — defer to SPEC-05
7. pons vs pons-* packages — defer to v0.5

## 9. Future possibilities

pons-live (SSE), pons-ws (WebSocket), pons-graphql, pons-auth, pons-test, pons-cli, pons-deploy

## 10. Impact on existing specifications

None. pons is a package, not a language change. No CONV, LEX, GRAM, TYPE, LIR rules added or modified.

## 11. Implementation plan

Phase 0 (now): RFC, README, good first issues
Phase 1 (M2): app.lo, router.lo, request.lo, response.lo, hello-world example
Phase 2 (M3): i18n.lo, multilingual routes, db.lo, visitors-crud example
Phase 3 (M4): middleware, templates, static files, 30 tests
Phase 4 (post-1.0): separate repo, publish to registries

## 12. How to discuss this RFC

Open PR with [RFC-0002] in title. General discussion on GitHub Discussions. Substantive changes via new RFC. Open until 2027-01-15.

## 13. Change log

| Date | Author | Change |
|------|--------|--------|
| 2026-11-23 | Claudiu Groza | Initial draft |

End of RFC-0002 (Draft).