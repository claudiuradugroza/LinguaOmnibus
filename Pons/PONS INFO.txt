Language is not an obstacle — it is a bridge.
Lingua non est impedimentum — est pons.

Filozofia pons — extinde promisiunea limbajului:
Lingua Omnibus promite	Pons promite
Programatorul scrie cod în limba lui	Utilizatorul final vede aplicația în limba lui
Aceeași semantică, sintaxă diferită	Aceeași aplicație, interfață diferită
Programatorul nu învață engleză	Utilizatorul nu învață engleză

Asta e diferențiatorul uriaș. Majoritatea framework-urilor web tratează i18n ca un afterthought (gettext, i18next, rails-i18n). Pons îl tratează ca first-class — rute, validări, mesaje de eroare, toate traduse nativ.


Cum arată concret:
1. Hello, World — în două limbi, același binar

Engleză: 
#lingua: EN
import pons

app = pons.new_app()

@app.get("/")
function index(req: Request) returns Response with [IO]:
    return pons.html("<h1>Hello, world!</h1>")

app.listen(8080)

Română:

#lingua: RO
importă pons

app = pons.aplicație_nouă()

@app.obține("/")
funcție index(cerere: Cerere) returnează Răspuns cu [IO]:
    returnează pons.html("<h1>Salut, lume!</h1>")

app.ascultă(8080)

Aceeași rută, același binar. Dovada că promisiunea limbajului se menține.

2. Aici e magia. Vrei ca utilizatorul francez să vadă /a-propos, cel român /despre, cel englez /about — dar un singur handler:
#lingua: EN
@app.get(route: {
    "en": "/about",
    "ro": "/despre",
    "fr": "/a-propos"
})
function about(req: Request) returns Response with [IO]:
    return pons.render("about.html", {
        title: pons.t("about.title"),
        body:  pons.t("about.body")
    })
	

Pons detectează limba din Accept-Language, cookie, sau query param ?lang=ro și servește ruta corectă. Un singur handler, trei URL-uri publice, zero duplicare de logică.

3. LinguaDB nativ — modele fără ORM

#lingua: EN
import pons
import pons.db

struct Visitor:
    id:      Integer primary_key auto_increment
    name:    String(100) not_null
    email:   String(255) not_null unique
    age:     Integer
    created: Time default NOW()

@app.post("/visitors")
function create_visitor(req: Request) returns Response
    with [Database, IO]:

    var body = req.json::<VisitorInput>()?

    validate(body) match:
        Err(e):
            return pons.json_error(422, e)
        Ok(valid):
            var visitor = db.insert(Visitor, valid)?
            return pons.json(201, visitor)

@app.get("/visitors/{id: Integer}")
function show_visitor(req: Request) returns Response with [Database]:
    var visitor = db.find(Visitor, req.path.id)?
    return pons.json(200, visitor)
	
Niciun ORM. Nicio migrare pentru MVP. Schema e definită în cod, verificată la compilare, db.insert și db.find sunt type-safe.

#lingua: EN
function validate(input: VisitorInput) returns Result<VisitorInput, ValidationError>:
    if not input.email.contains("@"):
        return Err(ValidationError(
            field: "email",
            code:  "email.invalid",
            key:   "errors.email_invalid"
        ))
    if input.age < 0 or input.age > 150:
        return Err(ValidationError(
            field: "age",
            code:  "age.out_of_range",
            key:   "errors.age_range"
        ))
    return Ok(input)
	
Catalog de traduceri (locales/):
# locales/en.json
{
  "errors.email_invalid": "Please enter a valid email address.",
  "errors.age_range":     "Age must be between 0 and 150."
}

# locales/ro.json
{
  "errors.email_invalid": "Vă rugăm introduceți o adresă de email validă.",
  "errors.age_range":     "Vârsta trebuie să fie între 0 și 150."
}

# locales/ja.json
{
  "errors.email_invalid": "有効なメールアドレスを入力してください。",
  "errors.age_range":     "年齢は0〜150の範囲で入力してください。"
}

Programatorul scrie validarea o singură dată, în orice limbă. Utilizatorul final primește mesajul în limba lui. Zero duplicare. Zero if-uri pe limbă.

5. Effects system — totul explicit
#lingua: EN
@app.post("/process-payment")
function process_payment(req: Request) returns Response
    with [IO, Database, Network]:          // ← explicit!
    var order = db.find(Order, req.path.id)?
    var result = payment_gateway.charge(order)?
    db.update(order, status: "paid")?
    return pons.json(200, { status: "ok" })
	
Dintr-o privire la semnătură, știi exact ce efecte secundare are handler-ul. Asta nu există în niciun framework mainstream.

6. Unde trăiește framework-ul

Aceasta e decizia arhitecturală importantă. Există trei opțiuni:
Opțiune	Locație	Pro	Contra
A. În spec	spec/14-framework.md	Un singur loc	Încalcă Principiul 2 (framework ≠ limbaj)
B. În monorepo	packages/pons/	Evoluează cu limbajul	Cuplat cu release-urile limbajului
C. Repo separat	github.com/.../pons	Independent, curat	Greu până când lopm funcționează

Recomandarea mea: B → C.

    Acum (M0–M2): în monorepo, la packages/pons/, pentru că lopm nu e gata. Se dezvoltă în paralel cu impl/loc.

    După M3: când lopm e funcțional, migrează în repo separat pons și distribuie-l ca pachet normal.

Asta respectă Principiul 2 (spec-driven) — framework-ul nu e în spec. Și Principiul 5 (federated packages) — pons e un pachet ca oricare altul, doar că e „first-party".
Structura packages/pons/ (în monorepo, acum).

packages/
└── pons/
    ├── README.md
    ├── loc.toml                    # manifest (ca orice pachet LO)
    ├── src/
    │   ├── main.lo                 # entry point
    │   ├── app.lo                  # App builder, listen()
    │   ├── router.lo               # typed routes + multilingual
    │   ├── request.lo              # Request type
    │   ├── response.lo             # Response type
    │   ├── middleware.lo           # middleware chain
    │   ├── handlers.lo             # handler trait
    │   ├── validate.lo             # validation helpers
    │   ├── i18n.lo                 # translation catalog + t()
    │   ├── db.lo                   # LinguaDB integration
    │   └── html.lo                 # HTML rendering (v0.2)
    ├── locales/
    │   ├── en.json                 # default catalog
    │   ├── ro.json
    │   ├── fr.json
    │   ├── de.json
    │   ├── es.json
    │   └── ja.json
    ├── tests/
    │   ├── router_test.lo
    │   ├── validate_test.lo
    │   └── i18n_test.lo
    └── examples/
        ├── hello-world/
        ├── visitors-crud/
        └── multilingual-blog/
		
Observație critică: pons este scris în Lingua Omnibus (fișiere .lo), nu în Rust. Asta e dovada că limbajul e suficient de matur pentru a-și scrie propriul framework. Dacă nu poți scrie pons în LO, limbajul nu e gata de lansat.

7. Cum se leagă de componentele existente

Componentă	Rol în pons
SPEC-01 (Lexical)	pons folosește doar tokenii standard
SPEC-02 (Grammar)	struct Visitor, match, with [...] — toate din gramatica existentă
SPEC-03 (Types)	Result<T,E>, Option<T>, types liniare pentru conexiuni DB
SPEC-05 (Effects)	with [IO, Database, Network] — pons e cel mai bun demo pentru effects
SPEC-08 (LinguaDB)	db.insert, db.find, db.update — API-ul pe care pons îl expune
SPEC-09 (Toolchain)	loc run pornește serverul, lopm add pons îl instalează
SPEC-10 (Stdlib)	std.io, std.net — dependențe ale pons
SPEC-11 (Security)	Capabilities pentru Network — pons le respectă

Nicio regulă nouă de limbaj nu e necesară. Pons e construit peste specificație, nu în ea. Asta e exact ce vrem.

8. Roadmap pons — de la MVP la v1.0

Versiune	Ce include	Când	Dependențe
v0.1 (MVP)	Router, handlers, JSON, with [IO], dev server	După M2	impl/loc funcțional
v0.2	LinguaDB integration, validation cu mesaje	După M2	SPEC-08 + loc DB
v0.3	Multilingual routes, catalog t(), locale detection	După M2	v0.2
v0.4	Middleware, static files, HTML templates	După M3	v0.3
v0.5	Auth helpers, sessions, cookies	După M3	v0.4
v1.0	API stable, book publicat, 3 example apps	După M4	v0.5 + tests

Fiecare versiune are un demo funcțional. Nu adăugăm features fără să avem un program care le arată.

Framework-ul nu poate precede limbajul. Ordinea corectă:
1. impl/loc — lexer care trece conformance/lex/         ✅ necesar
2. impl/loc — parser care trece conformance/gram/       ✅ necesar
3. impl/loc — backend x86_64 pentru Hello, World        ✅ necesar
4. stdlib/std.net — TCP listener + HTTP parser minimal  ✅ necesar
5. stdlib/std.io — print, read                          ✅ necesar
6. packages/pons — primul handler HTTP                  ⬅ aici începe pons

Estimare realistă: pons v0.1 nu poate exista înainte de M2. Dar design-ul poate fi scris acum, în paralel cu SPEC-03 și impl/loc.

10. Ce poți face concret în următoarele 2 săptămâni

1 — decizia + documentul:

    Creezi packages/pons/README.md cu filozofia și exemplul Hello, World

    Deschizi un RFC: rfcs/0002-pons-web-framework.md — propunerea formală

    Anunți în Discussions: „Pons v0.1 design is open for feedback"

    Adaugi pons la secțiunea „Ecosystem" din README-ul principal

2 — scheletul:

    packages/pons/loc.toml — manifest

    packages/pons/src/app.lo — stub cu new_app() și listen()

    packages/pons/src/router.lo — stub cu @app.get()

    packages/pons/src/response.lo — stub cu pons.html(), pons.json()

    packages/pons/tests/router_test.lo — un test care compilează dar nu rulează încă

    Adaugi 3 good first issue-uri: „Implement pons.html()", „Implement route matching", „Write example visitors-crud app"

Asta arată că proiectul are viziune + direcție + pași concreți. Contribuitorii vin la ceva ce pare în mișcare.





----------------------------------------------------------------------------
	
	RFC-0002: pons — A Multilingual-First Web Framework for Lingua Omnibus

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

1. Summary

This RFC proposes pons, a multilingual-first web framework for Lingua Omnibus, written in Lingua Omnibus itself. pons treats natural language as a first-class concern not only for the programmer (as the language already does) but for the end user of the applications built with it — routes, validation messages, error pages, and templates all adapt to the user's language without duplicating application logic.

pons is not part of the language specification. It is a normal package, distributed via lopm, forkable, replaceable, and subject to no special privilege. It exists to prove that Lingua Omnibus is expressive enough to host its own ecosystem.
2. Motivation
2.1 The problem with existing frameworks

Every mainstream web framework treats internationalization (i18n) as a plugin, not a premise:

    Rails has rails-i18n; routes are defined once, translations are looked up at render time

    Phoenix has gettext; the same handler serves all languages, with Gettext.gettext/1 sprinkled throughout

    Django has {% trans %} template tags; the URL structure is language-agnostic but the URL strings themselves are English

    Express/Fastify have zero built-in i18n; every project reinvents it

The result is that multilingual applications are structurally second-class. Programmers write if (user.lang === 'ro') scattered across the codebase, or they accept that all routes will be in English. Every i18n library is a patch on top of an architecture that assumed English.
2.2 The Lingua Omnibus opportunity

Lingua Omnibus was created to make the programmer's natural language first-class (SPEC-00 §1, CONV-074). pons extends that promise to the end user's natural language.

Consider a simple "about" page. Today, in any framework:
python:
@app.route("/about")           # English URL, hard-coded
def about():
    return render("about.html", title=t("about.title"))
	
The URL is English. The route is English. If a French user visits, they get /about, in French. This is a structural asymmetry that pons eliminates:
LO:
@app.get(route: {
    "en": "/about",
    "ro": "/despre",
    "fr": "/a-propos"
})
function about(req: Request) returns Response with [IO]:
    return pons.render("about.html", {
        title: pons.t("about.title"),
        body:  pons.t("about.body")
    })
	
One handler. Three URLs. Zero duplication. The compiler verifies that every route has at least one language; pons warns if a route is missing a language declared in loc.toml.

2.3 Why now

Three preconditions are now met:

    SPEC-00, 01, 02 are stable drafts. Grammar, lexical structure, and unified AST are normatively specified.

    The unified AST contract (GRAM-053) guarantees that the same handler written in EN and RO produces byte-identical LIR — the framework inherits this guarantee for free.

    The 13 constitutional principles already commit the project to interop, federation, and open formats — pons is a natural extension, not a new direction.

pons cannot be implemented until impl/loc and stdlib/std.net are functional. But it can be designed, specified, and stubbed now, so that when loc reaches M2, pons is ready to run.

3. Guide-level explanation

This section describes how a user would interact with pons if it existed today.
3.1 Installation
bash
lopm add pons

pons is a normal package. It has a loc.toml, a version, a SemVer. It is distributed through any lopm registry, including self-hosted ones (CONV-080, Principle 5). There is no "official" channel that a user must trust.

3.2 The canonical example

#lingua: EN
import pons

app = pons.new_app()

@app.get("/")
function index(req: Request) returns Response with [IO]:
    return pons.html("<h1>Hello, world!</h1>")

function main() returns Result<Unit, AppError> with [IO, Network]:
    return app.listen(8080)
	
#lingua: RO
importă pons

app = pons.aplicație_nouă()

@app.obține("/")
funcție index(cerere: Cerere) returnează Răspuns cu [IO]:
    returnează pons.html("<h1>Salut, lume!</h1>")

funcție principal() returnează Result<Unit, AppError> cu [IO, Network]:
    returnează app.ascultă(8080)
	
Both files compile to byte-identical binaries (via CONV-042), differing only in which pon response bytes are sent to the client. This is the framework's promise.

3.3 Multilingual routes

Routes can be declared with per-language URLs:

@app.get(route: { "en": "/about", "ro": "/despre", "fr": "/a-propos" })
function about(req: Request) returns Response with [IO]:
    return pons.render("about.html")
	
pons resolves the incoming request in this order:

    Explicit query parameter ?lang=ro — always wins

    Cookie pons_lang=ro — set by pons.set_lang/2

    Accept-Language header — parsed per RFC 7231 §5.3.5

    Default language — declared in loc.toml under [pons] default_lang = "en"

If none of the three route URLs match, pons returns a 406 Not Acceptable with a body that lists the available languages and their URLs. If the request matches a route but not in the user's preferred language, pons returns a 302 Found to the equivalent route in the fallback language, with a Vary: Accept-Language header.

3.4 LinguaDB integration — no ORM

struct Visitor:
    id:      Integer primary_key auto_increment
    name:    String(100) not_null
    email:   String(255) not_null unique
    age:     Integer
    created: Time default NOW()

@app.post("/visitors")
function create_visitor(req: Request) returns Response
    with [Database, IO]:
    var body = req.json::<VisitorInput>()?
    var validated = validate(body)?
    var visitor = db.insert(Visitor, validated)?
    return pons.json(201, visitor)

@app.get("/visitors/{id: Integer}")
function show_visitor(req: Request) returns Response with [Database]:
    var visitor = db.find(Visitor, req.path.id)?
    return pons.json(200, visitor)
	
db.insert and db.find are thin wrappers over LinguaDB (SPEC-08) — the storage engine is embedded, the schema is compile-time verified, and SQL injection is impossible by construction (implicit parameterization, SEC-030 in the future SPEC-11).

3.5 Validation with end-user messages

function validate(input: VisitorInput) returns Result<VisitorInput, ValidationError>:
    if not input.email.contains("@"):
        return Err(ValidationError(field: "email", code: "email.invalid"))
    if input.age < 0 or input.age > 150:
        return Err(ValidationError(field: "age", code: "age.out_of_range"))
    return Ok(input)
	
# locales/en.json
{
  "errors.email.invalid":     "Please enter a valid email address.",
  "errors.age.out_of_range":  "Age must be between 0 and 150."
}

# locales/ro.json
{
  "errors.email.invalid":     "Vă rugăm introduceți o adresă de email validă.",
  "errors.age.out_of_range":  "Vârsta trebuie să fie între 0 și 150."
}

When pons serializes the ValidationError into a 422 response, it looks up the code in the active locale and returns the translated message. The programmer writes validation once, in any language. The user reads errors in their language.

3.6 Effects are visible in the signature

@app.post("/process-payment")
function process_payment(req: Request) returns Response
    with [IO, Database, Network]:
    var order  = db.find(Order, req.path.id)?
    var result = payment_gateway.charge(order)?
    db.update(order, status: "paid")?
    return pons.json(200, { status: "ok" })
	
The with [IO, Database, Network] clause is a compile-time contract. pons uses it for two things:

    Security: a handler without Network cannot open an outbound socket, enforced by the effects system (SPEC-05, planned).

    Introspection: pons can generate a per-route effects report (pons effects) listing every handler's declared side effects — useful for audits, sandboxing policies, and rate limiting.

This is a feature no mainstream framework offers. It falls out naturally from Lingua Omnibus's type system.

4. Reference-level explanation
4.1 Where pons lives

Phase 1 (spec-0.1 to M2): inside the monorepo at packages/pons/.

Phase 2 (post-M3): migrates to its own repository github.com/lingua-omnibus/pons and is distributed via lopm like any other package.

Rationale: during Phase 1, lopm is not yet functional enough to distribute pons cleanly. But pons must be developed in parallel with impl/loc, because pons is the best way to discover language deficiencies before spec-1.0 freezes the language.

This is not an exception to Principle 2 (CONV-040, CONV-042): pons is not part of the spec, is not normative, and does not enjoy special status. It simply lives in the same repo during the bootstrap period. When it moves out, no downstream user notices — the package name and API remain unchanged.
4.2 Package layout

packages/pons/
├── loc.toml                          # manifest
├── README.md
├── LICENSE                           # Apache 2.0
├── src/
│   ├── main.lo                       # public API re-exports
│   ├── app.lo                        # App type, new_app, listen
│   ├── router.lo                     # route table, method dispatch
│   ├── request.lo                    # Request type
│   ├── response.lo                   # Response type
│   ├── middleware.lo                 # middleware trait + chain
│   ├── handler.lo                    # Handler trait
│   ├── validate.lo                   # ValidationError, validate helpers
│   ├── i18n.lo                       # locale catalog, t(), set_lang
│   ├── db.lo                         # LinguaDB thin wrappers
│   ├── html.lo                       # HTML rendering (v0.4+)
│   └── template.lo                   # template engine (v0.4+)
├── locales/
│   ├── en.json
│   ├── ro.json
│   ├── fr.json
│   ├── de.json
│   ├── es.json
│   └── ja.json
├── tests/
│   ├── router_test.lo
│   ├── validate_test.lo
│   ├── i18n_test.lo
│   └── db_test.lo
└── examples/
    ├── hello-world/
    ├── visitors-crud/
    └── multilingual-blog/
	
pons is written entirely in .lo files. Every feature that pons uses must already exist in the language. If pons needs something the language lacks, that is evidence that the language spec is incomplete — the correct response is an RFC against the language, not a special compiler extension for pons.
4.3 Public API (v0.1 minimum)

Types:
pub struct App { ... }
pub struct Request { ... }
pub struct Response { ... }
pub struct ValidationError { field: String, code: String }
pub type Handler = Function(Request) -> Response

Functions:

Function	Signature	Stability
pons.new_app	() -> App	stable (v0.1)
app.listen	(port: Integer) -> Result<Unit, AppError> with [IO, Network]	stable (v0.1)
app.get	(route: Route, handler: Handler) -> Unit	stable (v0.1)
app.post	same	stable (v0.1)
app.put	same	stable (v0.1)
app.delete	same	stable (v0.1)
pons.html	(content: String) -> Response	stable (v0.1)
pons.json	(status: Integer, body: T) -> Response	stable (v0.1)
pons.t	(key: String) -> String	stable (v0.3)
pons.set_lang	(code: String) -> Unit	stable (v0.3)
db.insert	(T, T) -> Result<T, DbError> with [Database]	stable (v0.2)
db.find	(type T, id) -> Result<T, DbError> with [Database]	stable (v0.2)
db.update	(T, T) -> Result<Unit, DbError> with [Database]	stable (v0.2)

The route parameter accepts either a String (single-language) or a Map<String, String> (multilingual). Both are declared in the type system — no dynamic dispatch, no runtime inspection.
4.4 Route resolution algorithm

Given an incoming Request:

    Compute accept_langs = parse_accept_language(req.headers["accept-language"])

    Compute requested_lang = req.query["lang"] ?? req.cookies["pons_lang"] ?? accept_langs[0] ?? config.default_lang

    For each route in the table, check if req.method matches AND req.path matches any of the route's URLs.

    If a match is found in requested_lang, dispatch the handler.

    If a match is found in a different language, return 302 to the equivalent URL in requested_lang.

    If no route matches at all, return 404.

    If requested_lang is not in config.languages, return 406 with available languages listed.

This algorithm is fully deterministic and specifiable in ~30 lines of pons code. No hidden magic, no "convention over configuration" surprises.
4.5 Locale catalog format

locales/*.json files are plain JSON (not .lo), because they are frequently generated from translation platforms and because forcing translators to learn a programming language would be hostile to the very people pons is meant to empower.

json:
{
  "$schema": "https://lingua-omnibus.org/schema/pons-locale-0.1.json",
  "lang": "ro",
  "version": "2026-01-01",
  "messages": {
    "errors.email.invalid":    "Vă rugăm introduceți o adresă de email validă.",
    "errors.age.out_of_range": "Vârsta trebuie să fie între 0 și 150.",
    "ui.buttons.submit":       "Trimite",
    "ui.buttons.cancel":       "Anulează"
  }
}

Missing translations fall back to the default language (declared in loc.toml). If a message key is missing in every language, pons returns the key itself (never a crash — a common best practice from gettext).
4.6 Relationship to LinguaDB

pons.db is a thin wrapper over LinguaDB (SPEC-08). It does not reimplement the storage engine, does not cache queries, and does not add an ORM layer. Its only jobs are:

    Convert struct definitions into create table statements at compile time

    Wrap insert, find, update, delete in typed, parameterized calls

    Translate LinguaDB errors into DbError (a stable Result type)

This means: you can use LinguaDB directly if you don't want pons.db. pons is not a gatekeeper.
4.7 What pons deliberately does NOT include

At v0.1:

    No template engine — pons.html accepts raw HTML strings. Templates come in v0.4.

    No authentication — deferred to a separate pons-auth package (v0.5+).

    No static file serving — deferred to v0.4.

    No WebSocket support — deferred to v0.5.

    No database migrations — LinguaDB handles schema at compile time; runtime migrations are a separate concern for a pons-migrate package (v0.6+).

    No admin panel — a full framework is out of scope. pons is a web framework, not a CMS.

This is a deliberate strategy: ship a minimal, coherent core rather than a kitchen-sink framework that nobody can understand.
5. Drawbacks
5.1 Coupling to LinguaDB

pons.db depends on LinguaDB (SPEC-08), which is not yet specified. If SPEC-08 changes significantly, pons.db may need to change. This is a known risk, mitigated by keeping pons.db as a thin wrapper (see §4.6).
5.2 Bootstrapping risk

pons cannot be tested until impl/loc works end-to-end. During M0–M1, pons exists only as a stub. If impl/loc slips, pons slips.
5.3 Scope creep

Every framework wants to become a platform. pons explicitly refuses this path (§4.7). The risk is real: contributors may want "just one more feature". The mitigation is the v0.1 API freeze in §4.3 — additions must go through RFCs.
5.4 Fragmentation with future frameworks

Other contributors may want a different framework (pons vs. katharos vs. whatever). This is fine — it is exactly what Principle 5 (federated packages) is for. pons has no monopoly. If katharos becomes more popular, pons users can migrate.
5.5 No prior art for "multilingual routes"

To the best of our knowledge, no mainstream framework offers URL-per-language with a single handler and zero duplication. This is pons's biggest differentiator but also its biggest unknown. If the ergonomics turn out to be bad in practice, we may need to revisit the API before v1.0.
6. Rationale and alternatives
6.1 Alternative A: pons in Rust, binding to loc

Rejected. If pons is written in Rust, it cannot be a proof that Lingua Omnibus is expressive enough. It would also create an FFI boundary that defeats the "no new features for pons" principle.
6.2 Alternative B: pons as part of the spec

Rejected. Violates Principle 2 (spec-driven, not implementation-driven). The spec should describe the language, not the ecosystem. Frameworks evolve faster than languages; tying them together hurts both.
6.3 Alternative C: pons written in Python or TypeScript, transpiled

Rejected. Destroys the self-hosting proof, adds a dependency on another language's ecosystem, and violates Principle 1 (zero vendor lock-in) because the framework would be tied to the transpiler's runtime.
6.4 Alternative D: contribute to an existing multilingual framework

Rejected. No existing framework treats multilingual as first-class (§2.1). rails-i18n, phoenix-gettext, django-i18n all treat language as a rendering concern, not a routing concern. pons is genuinely novel.
6.5 Alternative E: build on top of an existing web server library

Considered but deferred. At v0.1, pons implements its own minimal HTTP/1.1 parser (~500 lines). If stdlib/std.net grows a robust HTTP server, pons will switch to it. This is a build vs. borrow decision that can be revisited at v0.5.
7. Prior art

pons draws from several projects. None of them are direct competitors; all of them inform the design.
Project	Language	What we borrow	What we reject
Rails	Ruby	Convention-over-configuration feel; @app.get DSL	ORM coupling; i18n as afterthought
Phoenix	Elixir	Channels-inspired typed handlers; socket model	Erlang-specific tooling; gettext model
Axum	Rust	Typed extractors; explicit effects	Macro-heavy DSL; async complexity
Django	Python	Admin-optional; batteries-included feel	Monolithic; ORM mandatory
FastAPI	Python	Auto-generated OpenAPI; type-driven	Python-typed; no effects
Laravel	PHP	Rich ecosystem; route() helpers	PHP-only; heavy magic
Spring Boot	Java	Annotation-driven config	JVM weight; enterprise bloat
Next.js	JS/TS	File-based routing (rejected here)	JS-only; Vercel coupling
Hono	JS/TS	Minimal core; middleware-first	JS-only; no types

What's genuinely novel in pons:

    Multilingual routes — no direct precedent

    Native database with compile-time schema — closest is Diesel (Rust), but it's not embedded and not multilingual

    Effects visible in route signatures — closest is Koka, but Koka isn't a web framework

8. Unresolved questions

    Route declaration syntax. @app.get(route: { "en": "/about", ... }) is verbose. Alternative: @app.get("/about", lang: ["en", "ro", "fr"]) with automatic URL slug from a translation table. Decision needed by v0.2.

    Template language. v0.4 introduces templates. Should they be .lo files with a @template pragma, or a separate DSL, or a JSON-based declarative format? Deferred to a future RFC.

    Locale detection precedence. §4.4 uses query > cookie > header > default. Alternative: cookie > query > header > default. Needs user feedback.

    Missing-language behavior. When a route has no translation for the requested language, we currently redirect. Alternative: render in default language with a lang attribute change. Needs usability testing.

    Session management. v0.5 introduces sessions. Cookie-based? Server-side? JWT? Deferred.

    Error handling at framework level. Does pons catch panics in handlers and return 500? Or does it let them propagate? Rust's axum catches; PHP's Laravel catches; Go's net/http catches. Defer to SPEC-05 (effects).

    pons vs pons-* packages. Should auth, migrations, templates be separate packages or subpackages? Rails uses subpackages; Axum uses separate crates. Defer to v0.5.

9. Future possibilities

Once pons reaches v1.0, these become natural extensions:

    pons-live — server-sent events (SSE) with typed channels

    pons-ws — WebSocket support

    pons-graphql — GraphQL over the same handler model

    pons-auth — batteries-included authentication

    pons-test — request mocking, assertion helpers

    pons-cli — project scaffolding (pons new blog, pons generate model)

    pons-deploy — one-command deployment to a target (Dockerfile generation, WASM build, etc.)

None of these are in scope for the current RFC. They are mentioned to show direction, not to promise delivery.
10. Impact on existing specifications

None. This RFC proposes a package, not a language change. No CONV, LEX, GRAM, TYPE, LIR, or other rules are added or modified.

pons relies on existing specs and inherits their guarantees for free:
Spec	How pons uses it
SPEC-00 §3.3 (glossary)	Every user-facing message uses glossary terms
SPEC-01 §2.9 (directives)	#lingua: selects the source language of the app
SPEC-02 §4.1 (grammar)	All handlers use the reference grammar; no extensions
SPEC-02 GRAM-053 (unified AST)	pons inherits byte-identical AST across source languages
SPEC-03 (planned)	Result, Option, Map<String,String> for routes
SPEC-05 (planned)	with [IO, Database, Network] effects
SPEC-08 (planned)	LinguaDB engine, schema verification
SPEC-09 (planned)	lopm add pons, loc run
SPEC-10 (planned)	std.net, std.io, std.json

If any of these specs change in ways that break pons, the fix is in the framework, not in the spec.
11. Implementation plan
Phase 0 (now — design)

    ☑

    This RFC
    □

    packages/pons/README.md (philosophy + quick start)
    □

    3 good first issues open
    □

    Discussion thread on GitHub Discussions

Phase 1 (M2 — first runnable code)

Dependencies: impl/loc compiles Hello World; stdlib/std.net has a TCP listener.

    □

    packages/pons/src/app.lo — new_app, listen
    □

    packages/pons/src/router.lo — single-language routes only
    □

    packages/pons/src/request.lo — minimal HTTP request parser
    □

    packages/pons/src/response.lo — pons.html, pons.json
    □

    packages/pons/examples/hello-world/ — works in EN and RO
    □

    packages/pons/tests/router_test.lo — 5 tests, all passing

Phase 2 (M3 — multilingual + DB)

Dependencies: SPEC-08 LinguaDB functional.

    □

    packages/pons/src/i18n.lo — locale catalog, pons.t, set_lang
    □

    Multilingual routes (Map<String, String> route syntax)
    □

    Locale detection (§4.4)
    □

    packages/pons/src/db.lo — thin LinguaDB wrappers
    □

    packages/pons/examples/visitors-crud/
    □

    15 tests, all passing

Phase 3 (M4 — production-ready v0.5)

    □

    Middleware system
    □

    Static file serving
    □

    HTML template engine
    □

    pons-test mocking helpers
    □

    Documentation in docs/book/11-pons-framework.md
    □

    30 tests, all passing

Phase 4 (post-1.0 — separate repo)

    □

    Migrate to github.com/lingua-omnibus/pons
    □

    Publish to at least two lopm registries (federated proof)
    □

    3 real-world apps built with pons
    □

    pons --version reports spec compatibility per CONV-034

12. How to discuss this RFC

    Inline comments: open a PR on this file with [RFC-0002] in the title

    General discussion: GitHub Discussions thread (link to be added)

    Substantive changes: propose a new RFC (RFC-0003, etc.) rather than editing this one after it reaches Accepted

This RFC is open until at least 2027-01-15, after which it will either move to Accepted, Withdrawn, or Superseded. Feedback is welcome from anyone — you do not need to be a maintainer.
13. Change log
Date	Author	Change
2026-11-23	Claudiu Groza	Initial draft

End of RFC-0002 (Draft).
Notă (în română, pentru tine)

Am scris RFC-ul în engleză pentru că toate spec-urile normative sunt în engleză (per CONV-074 din SPEC-00 — glosarul și regulile normative sunt în engleză; traducerile sunt pentru mesajele human-facing, nu pentru documentele normative). Dacă vrei ca RFC-urile să fie bilingve, trebuie mai întâi un RFC care să schimbe această convenție.

Ce e în document:

    Secțiunea 1 (Summary) — o propoziție clară: pons e un pachet, nu parte din limbaj

    Secțiunea 2 (Motivation) — de ce framework-urile existente eșuează la i18n, cu exemplu concret de cod

    Secțiunea 3 (Guide-level) — cum arată pons pentru un utilizator, cu 6 exemple de cod reale

    Secțiunea 4 (Reference-level) — layout, API, algoritm de rutare, format locale, ce NU include

    Secțiunea 5 (Drawbacks) — 5 riscuri oneste (coupling, bootstrapping, scope creep, fragmentare, lipsa prior art)

    Secțiunea 6 (Alternatives) — 5 alternative respinse, cu rationale

    Secțiunea 7 (Prior art) — 9 framework-uri comparate, ce împrumutăm și ce respingem

    Secțiunea 8 (Unresolved) — 7 întrebări deschise, cu termene

    Secțiunea 9 (Future) — extensii viitoare (deliberat NU promisiuni)

    Secțiunea 10 (Impact on specs) — explicit: zero modificări la spec-uri

    Secțiunea 11 (Implementation) — 4 faze cu checkbox-uri concrete

    Secțiunea 12 (Discussion) — cum se discută, până la 2027-01-15

Fișierul e gata să-l pui la rfcs/0002-pons-web-framework.md în repo. Următorii pași recomandați:

    Push fișierul — commit cu mesaj docs(rfcs): propose RFC-0002 pons web framework

    Deschide un issue tracking — [RFC-0002] pons — tracking issue cu link la fișier

    Deschide un Discussion — [RFC-0002] pons: feedback wanted cu link la fișier

    Adaugă la README-ul principal — o secțiune „Active RFCs" cu link la RFC-0002

    Creează packages/pons/README.md — versiunea scurtă, cu link la RFC complet


________________________________________________________________________

Hello, World — English

#lingua: EN
import pons

function index(req: Request) returns Response with [IO]:
    return pons.html("<h1>Hello, world!</h1>")

function main() returns Result<Unit, AppError> with [IO, Network]:
    var app = pons.new_app()
    app.get("/", index)
    return app.listen(8080)
	
Hello, World — Romanian

#lingua: RO
importă pons

funcție index(cerere: Cerere) returnează Răspuns cu [IO]:
    returnează pons.html("<h1>Salut, lume!</h1>")

funcție principal() returnează Result<Unit, AppError> cu [IO, Network]:
    variabilă app = pons.aplicație_nouă()
    app.obține("/", index)
    returnează app.ascultă(8080)
	
Both files compile to byte-identical binaries (per CONV-042). The only difference is which bytes reach the client. That is the promise of Lingua Omnibus, inherited by pons for free.

Run it:
bash
loc run src/main.lo
# pons listening on http://localhost:8080

Visit http://localhost:8080 → Hello, world! (or Salut, lume! — the source language doesn't matter for the output).

Core concepts
🌍 Multilingual routes

Declare one handler, register it in multiple languages:

#lingua: EN
function about(req: Request) returns Response with [IO]:
    return pons.render("about.html", {
        title: pons.t("about.title"),
        body:  pons.t("about.body")
    })

function main() returns Result<Unit, AppError> with [IO, Network]:
    var app = pons.new_app()
    app.get(route: { "en": "/about", "ro": "/despre", "fr": "/a-propos" }, handler: about)
    return app.listen(8080)
	
A French user visiting /a-propos gets the same handler, with pons.t() resolving to French. A Romanian user visiting /despre gets Romanian. No if lang == "ro" anywhere.

Resolution order (per RFC-0002 §4.4):

    Query parameter ?lang=ro

    Cookie pons_lang=ro

    Accept-Language header

    Default from loc.toml

🗄️ LinguaDB integration — no ORM
lo

struct Visitor:
    id:      Integer primary_key auto_increment
    name:    String(100) not_null
    email:   String(255) not_null unique
    age:     Integer
    created: Time default NOW()

function create_visitor(req: Request) returns Response with [Database, IO]:
    var body = req.json::<VisitorInput>()?
    var validated = validate(body)?
    var visitor = db.insert(Visitor, validated)?
    return pons.json(201, visitor)
	
The schema is verified at compile time. SQL injection is impossible by construction. There is no ORM — db.insert is a typed wrapper over LinguaDB (SPEC-08).

✉️ Validation with end-user messages

lo

function validate(input: VisitorInput) returns Result<VisitorInput, ValidationError>:
    if not input.email.contains("@"):
        return Err(ValidationError(field: "email", code: "email.invalid"))
    if input.age < 0 or input.age > 150:
        return Err(ValidationError(field: "age", code: "age.out_of_range"))
    return Ok(input)
	
The user sees a message from the locale catalog, not from the handler:

// locales/en.json
{ "errors.email.invalid": "Please enter a valid email address." }

// locales/ro.json
{ "errors.email.invalid": "Vă rugăm introduceți o adresă de email validă." }

// locales/ja.json
{ "errors.email.invalid": "有効なメールアドレスを入力してください。" }

Write the validation once. Users read errors in their language.

⚡ Effects visible in the signature

lo

function process_payment(req: Request) returns Response
    with [IO, Database, Network]:
    var order  = db.find(Order, req.path.id)?
    var result = payment_gateway.charge(order)?
    db.update(order, status: "paid")?
    return pons.json(200, { status: "ok" })
	
From the signature alone, you know exactly what side effects this handler has. pons uses this for:

    Security auditing — a handler without Network cannot open a socket

    Per-route effects reports — pons effects lists every handler's declared effects

    Sandbox policies — deny any route that declares Filesystem in production

This feature does not exist in any mainstream framework. It falls out of Lingua Omnibus's type system for free.
Status

pons is in design phase. This table is honest about what works today:
Component	Status	Milestone
RFC-0002 (design)	✅ Draft	now
README.md (this file)	✅	now
loc.toml manifest	📋 planned	M2
src/app.lo — new_app, listen	📋 planned	M2
src/router.lo — single-language routes	📋 planned	M2
src/request.lo, src/response.lo	📋 planned	M2
examples/hello-world/	📋 planned	M2
src/i18n.lo — multilingual routes	📋 planned	M3
src/db.lo — LinguaDB wrappers	📋 planned	M3
src/validate.lo	📋 planned	M3
examples/visitors-crud/	📋 planned	M3
Middleware, templates, static files	📋 planned	M4
Separate repo + lopm publish	📋 planned	post-1.0

v0.1 cannot ship before impl/loc reaches M2. The framework depends on the language being runnable. See RFC-0002 §11 for the full milestone plan.
Architecture

pons is written in .lo files, not Rust, not Python, not TypeScript. This is deliberate — it is the self-hosting proof.

packages/pons/
├── src/              ← all .lo files
├── locales/          ← JSON translation catalogs
├── tests/            ← .lo tests, run with `lotest`
└── examples/         ← runnable demo apps

Dependencies (all declared in loc.toml):
Dependency	Used for
std.io	print, read
std.net	TCP listener, HTTP parser
std.json	request body parsing, response serialization
std.time	timestamps, cookies
lingua-db	LinguaDB bindings (SPEC-08)

Prerequisites — pons requires the following specs to be implemented:

    SPEC-01, 02 (lexing, parsing) — the language itself

    SPEC-05 (effects) — for with [IO, Database, Network]

    SPEC-08 (LinguaDB) — for db.insert, db.find

    SPEC-09 (toolchain) — for loc run, lopm add

    SPEC-10 (stdlib) — for std.net, std.json

Decorator syntax note: pons's public examples use app.get(route: …, handler: …) registration style, which works with spec-0.1 grammar as-is. A decorator style (@app.get(…)) is proposed in RFC-0002 §8-Q1 and would require a small grammar extension (potential RFC-0003). Registration style is the safe default until that RFC is accepted.
Contributing

You do not need to be a Rust programmer to contribute to pons. You do not even need to be a programmer of any language other than Lingua Omnibus (and if you're not yet, the tutorial takes 2 hours).
Good first issues

If you want to contribute, here are five starter tasks, in increasing difficulty:

    Add a locale catalog — translate locales/en.json into your language. One file, ~20 keys. No programming required. Open a PR with locales/xx.json.

    Write a hello-world example — examples/hello-world/ needs an app in your language. Copy the EN version, translate the #lingua: directive and the keywords, verify it produces the same AST.

    Add a conformance test — each RFC-0002 §4.3 API function should have a test. Write one in tests/.

    Implement pons.html() — the simplest function in the codebase. It wraps a string in a Response. 10 lines of .lo.

    Implement route matching — src/router.lo needs a trie-based matcher. ~100 lines. The RFC §4.4 specifies the algorithm exactly.

Design discussions

    RFC-0002 — the design document. Comment on the RFC or open an issue.

    GitHub Discussions — for open-ended questions.

    7 unresolved questions — see RFC-0002 §8. These need community input before v0.2.

What we are NOT looking for (yet)

    Authentication — v0.5+

    WebSocket — v0.5+

    Templates — v0.4+

    Static file serving — v0.4+

    Admin panel — out of scope, forever

Adding features before the core is stable hurts the project. See RFC-0002 §4.7 for the reasoning.
Philosophical commitments

pons inherits the 13 constitutional principles of Lingua Omnibus (see spec/00 §2). Three are especially relevant:

    Principle 1 (zero vendor lock-in) — pons has no cloud service, no license server, no telemetry. You can deploy it to any host.

    Principle 5 (federated packages) — pons will be published to any lopm registry, not just an "official" one. Self-host for your company; it works identically.

    Principle 10 (Apache 2.0) — pons is Apache 2.0. Fork it, rename it, ship it commercially. The only thing you can't do is close it.

The framework has no BDFL. Decisions go through RFCs. Maintainers are elected. Fork rights are explicit.
License

Apache 2.0. See LICENSE.
See also

    RFC-0002 — pons web framework design — full specification

    SPEC-00 — Conventions — rules, glossary, governance

    SPEC-01 — Lexical Structure — tokens, Unicode, directives

    SPEC-02 — Grammar — reference grammar, unified AST

    Tutorial — learn Lingua Omnibus in 2 hours

<div align="center">

"Language is not an obstacle — it is a bridge."

pons · Latin for bridge · Apache 2.0 · written in Lingua Omnibus
</div> ```
🔧 PARTEA II — Master Prompt pentru generarea codului PONS

Acesta este promptul pe care îl dai unui LLM (Claude, GPT, DeepSeek) pentru a genera fiecare fișier .lo din packages/pons/src/. Este modular — îl rulezi o dată per fișier.
🔧 MASTER PROMPT (copy-paste)

# ROLE

You are a senior engineer implementing **pons**, the multilingual-first web
framework for Lingua Omnibus (LO). You write Lingua Omnibus source code (.lo
files) that is correct, minimal, and conformant to the specifications. You
have deep knowledge of:

- SPEC-00 (Conventions & Terminology), especially CONV-040, CONV-042
- SPEC-01 (Lexical Structure), especially LEX-090, LEX-091
- SPEC-02 (Grammar), especially GRAM-030…038 (declarations), GRAM-050…054 (AST)
- RFC-0002 (pons design), especially §4.3 (public API) and §4.4 (routing)

You do not invent language features. If a feature is needed but not in the
spec, you flag it as "⚠ RFC REQUIRED: <description>" and propose a workaround
using existing grammar.

# PROJECT CONTEXT (immutable)

Lingua Omnibus is a spec-driven, community-governed, multilingual programming
language. Programs written in different natural languages produce byte-identical
machine code (CONV-042). Adding a natural language is pure data (CONV-061);
adding a feature requires an RFC.

pons is a normal LO package. It is written entirely in .lo files. It has no
compiler extensions, no privileged access, and no special treatment. If pons
cannot be expressed in the current spec, the spec is wrong, not pons.

# NON-NEGOTIABLE RULES

Every .lo file you produce MUST:

1. **Compile under spec-0.1 grammar** — no decorators, no `async`, no `await`,
   no `try/catch`, no `null`, no `?` outside `return …?` position (GRAM-024),
   no chained comparisons (GRAM-022), no one-liner blocks (GRAM-036).
2. **Start with `#lingua: EN`** — English is the canonical source language
   for pons internals. A Romanian translation is a separate concern (locales/).
3. **Use only keywords in the language pack's `keywords.json`** — if a keyword
   you want is not declared, flag it as "⚠ UNREGISTERED KEYWORD: <kw>" and use
   an identifier instead.
4. **Declare all effects in the function signature** — `with [IO]`, `with
   [Database]`, `with [Network]`, `with [IO, Network]`, etc. A function without
   an effect clause MUST NOT perform that effect. The compiler enforces this.
5. **Return `Result<T, E>` for any operation that can fail** — never `panic`,
   never implicit exceptions, never `null`. Use `?` only in `return …?` position.
6. **Use `Option<T>` for optional values** — never `null`. Use `match` to
   unwrap. `None()` and `Some(v)` are the constructors.
7. **Keep functions under 40 lines** — if a function exceeds 40 lines, split it
   into named helpers. Every helper gets a doc comment.
8. **Use doc comments (`///`) on every `pub` item** — the first line is a
   one-sentence summary, then a blank `///`, then details. `lodoc` consumes
   these; they become the public API reference.
9. **Never reference language-specific identifiers in core logic** — no
   `if language == "ro"`. Language dispatch happens through `locales/*.json`
   and `pons.t()`, never through code branches.
10. **Match the RFC-0002 §4.3 signatures exactly** — if the RFC says
    `app.listen(port: Integer) -> Result<Unit, AppError> with [IO, Network]`,
    the signature in code MUST be exactly that. If you need to deviate, flag it
    as "⚠ API DEVIATION: <reason>" and propose an RFC amendment.

# TASK

Generate the file **`packages/pons/src/{FILENAME}`**.

The file's purpose is described in RFC-0002 §4.3 (public API) and §4.2 (layout).
If the filename is not in the RFC, treat it as an internal helper and explain
its role in a top-of-file comment.

# TEMPLATE (mandatory structure for every .lo file)


#lingua: EN
// {FILENAME} — {one-sentence purpose}
//
// Part of pons (RFC-0002).
// Spec target: spec-0.1
// Dependencies: {list of other .lo files in packages/pons/src/}
//
// See: rfcs/0002-pons-web-framework.md §{section}

import std.io
import std.net
import std.json
// ... other imports, alphabetically sorted

// ─────────────────────────────────────────────────────────────
// Public types
// ─────────────────────────────────────────────────────────────

/// {one-sentence summary of the type}
///
/// {optional detailed description}
pub struct {TypeName}:
{field}: {Type}
// ... more fields

// ─────────────────────────────────────────────────────────────
// Public functions
// ─────────────────────────────────────────────────────────────

/// {one-sentence summary}
///
/// # Parameters
/// - {param}: {description}
///
/// # Returns
/// {description of Ok and Err cases}
///
/// # Effects
/// {list of effects this function performs, and why}
pub function {name}({params}) returns {ReturnType}
with [{effects}]:
// implementation

// ─────────────────────────────────────────────────────────────
// Internal functions (not exported)
// ─────────────────────────────────────────────────────────────

function {helper_name}({params}) returns {ReturnType}:
// implementation


# STYLE RULES

- **Formatting**: 4-space indentation (per GRAM-010–015). No tabs.
- **Line length**: aim for ≤ 90 columns; hard limit 100.
- **Naming**: `snake_case` for functions and variables, `PascalCase` for types
  and struct fields (per SPEC-00 §3.3 glossary conventions).
- **Doc comments**: use `///` for public items, `//` for internal notes. Every
  `pub` item gets a doc comment.
- **Error types**: prefer a dedicated `struct XxxError` with a `code: String`
  field over bare `String` messages — codes are stable across locales.
- **Effect ordering**: when a function declares multiple effects, order them
  alphabetically: `with [Database, IO, Network]` not `with [IO, Network, Database]`.
- **Imports**: sorted alphabetically, one per line.
- **Constants**: `const UPPER_SNAKE = value` at the top of the file, before types.
- **No dead code**: if a helper is unused, remove it. `loc check` will flag it.

# ANTI-PATTERNS (reject these in your output)

- ❌ Using `@decorator` syntax — not in spec-0.1 (GRAM-030). Use registration.
- ❌ Using `null`, `None` without `Option<T>`, or any bare nullable.
- ❌ Throwing exceptions or using `try/catch` — use `Result<T, E>` and `?`.
- ❌ `if x { … } else { … }` on one line — GRAM-036 forbids one-liner blocks.
- ❌ `if x:` without `then:` — GRAM-035 requires `then`.
- ❌ `a < b < c` — GRAM-022 forbids chained comparison.
- ❌ Calling a network function without `with [Network]`.
- ❌ Calling a database function without `with [Database]`.
- ❌ Hard-coded language strings in code (`if lang == "ro"`) — use locales.
- ❌ Referring to a stdlib function that doesn't exist in SPEC-10's planned API.
- ❌ Inventing keywords (`module`, `class`, `interface` if not declared).
- ❌ 200-line functions without decomposition.
- ❌ Placeholder comments like `// TODO: implement` — every function must be
  either fully implemented or excluded from the file.
- ❌ Using `print()` for errors — use `Result` or `std.log`.

# OUTPUT FORMAT

Return a single .lo file, as plain text, with the exact structure from
the TEMPLATE. No markdown fences around it, no preamble, no "here is your
file". Start directly with `#lingua: EN`.

If the file requires a stdlib function that is not yet available
(e.g., `std.json::parse` in a milestone where SPEC-10 isn't complete), you
MUST flag it as:


// ⚠ STDLIB DEPENDENCY: std.json::parse
// Required by: this file
// Status: NOT YET SPECIFIED in SPEC-10
// Workaround: stub returns Err(NotImplemented) until SPEC-10 is finalized


and proceed with a stub that returns `Err("not_implemented")`. Never invent
a stdlib API — flag it instead.

# SELF-CHECK (before you output)

Run these checks mentally on your output and fix any failures:

1. Does every function have an effects clause if it performs effects?
2. Does every function returning a fallible operation use `Result<T, E>`?
3. Are all keywords used actually declared in the language pack?
4. Are all blocks indented by 4 spaces, with `:` before `INDENT`?
5. Are `if` conditions followed by `then:`?
6. Is every `pub` item documented with `///`?
7. Is every line ≤ 100 columns?
8. Are all imports sorted alphabetically?
9. Are all effects in a `with [...]` clause sorted alphabetically?
10. Does the file match the RFC-0002 §4.3 API exactly (or is there a flagged
    deviation)?
11. Is there any `TODO`, `unimplemented`, or `panic`?
12. Does the file start with `#lingua: EN`?

If any answer is "no", fix the file before outputting.

# INPUTS (fill in before running)

- FILENAME:      {e.g., app.lo}
- SPEC_SECTION:  {e.g., RFC-0002 §4.3, SPEC-02 §4.1}
- DEPENDENCIES:  {e.g., router.lo, request.lo, response.lo}
- MILESTONE:     {e.g., M2, M3, M4 — determines scope}
- CONSTRAINTS:   {any additional user constraints, e.g., "must pass tests/router_test.lo"}

# EXAMPLE RUN

FILENAME:      router.lo
SPEC_SECTION:  RFC-0002 §4.4 (routing algorithm)
DEPENDENCIES:  request.lo, response.lo, handler.lo
MILESTONE:     M2 (single-language routes only)
CONSTRAINTS:   must expose `app.get(path: String, handler: Handler)` and
               `app.post(path: String, handler: Handler)`; multilingual routes
               are deferred to M3.

[LLM generates packages/pons/src/router.lo]

Cum folosești promptul — workflow practic

Rulezi promptul o dată per fișier, în ordinea dependențelor. Fiecare output îl salvezi în packages/pons/src/, apoi îl dai ca DEPENDENCIES la următorul.

Pas	Rulezi cu	Output	Depinde de
1	FILENAME=handler.lo, MILESTONE=M2	Trait Handler	—
2	FILENAME=request.lo, MILESTONE=M2	Request type	—
3	FILENAME=response.lo, MILESTONE=M2	Response, pons.html, pons.json	request.lo
4	FILENAME=router.lo, MILESTONE=M2	Route table + matcher	handler.lo, request.lo, response.lo
5	FILENAME=app.lo, MILESTONE=M2	App, new_app, listen	router.lo
6	FILENAME=main.lo, MILESTONE=M2	Public re-exports	all above
7	FILENAME=i18n.lo, MILESTONE=M3	pons.t, set_lang, catalog	—
8	FILENAME=validate.lo, MILESTONE=M3	ValidationError	response.lo
9	FILENAME=db.lo, MILESTONE=M3	db.insert, db.find	LinguaDB
10	FILENAME=middleware.lo, MILESTONE=M4	Middleware chain	response.lo
11	FILENAME=html.lo, MILESTONE=M4	Template engine	response.lo


Regulă: nu treci la pasul N+1 până când loc check nu trece pe fișierul de la pasul N. Sanity check automat, nu manual.





	
	
	
	
	
	

    packages/pons/README.md — README-ul framework-ului, cu filozofie, quick start, roadmap

    packages/pons/loc.toml + scheletul src/ (app.lo, router.lo, response.lo) — stub-uri care compilează dar nu rulează

    packages/pons/examples/hello-world/ — exemplul canonic, cu #lingua: EN și #lingua: RO

    packages/pons/examples/visitors-crud/ — exemplul cu LinguaDB, validare, multilingv

    locales/en.json + locales/ro.json — cataloagele de traduceri pentru validare

    docs/book/11-pons-framework.md — capitolul din carte despre pons

    packages/pons/tests/ — suite-ul de teste planificat

Care e prioritatea ta? Dacă vrei să atragi contribuitori săptămâna asta, recomand combinația 1 + 2 + 3 (RFC + README + schelet). Asta arată lumii că proiectul are un plan serios pentru framework și că oricine poate începe să contribuie la el.










