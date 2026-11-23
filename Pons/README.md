<div align="center">

# pons

**The multilingual-first web framework for Lingua Omnibus.**

*Latin: pons, pontis — "bridge."*

[![spec](https://img.shields.io/badge/spec-0.1-blue)](../../spec/00-conventions.md)
[![rfc](https://img.shields.io/badge/RFC-0002-orange)](../../rfcs/0002-pons-web-framework.md)
[![status](https://img.shields.io/badge/status-design-yellow)](../../rfcs/0002-pons-web-framework.md)
[![license](https://img.shields.io/badge/license-Apache%202.0-green)](LICENSE)

</div>

---

## What is pons?

**pons** is a web framework written **entirely in Lingua Omnibus**. It exists to prove two things:

1. **Lingua Omnibus is expressive enough** to host its own ecosystem — if you can't write a web framework in a language, the language isn't ready.
2. **Multilingual does not stop at the programmer.** Your code is in Romanian. Your **users** should see the app in Romanian too — not just the messages, but the **URLs, the routes, the error pages.**

pons is **not** part of the language specification. It is a normal package, distributed via `lopm`, forkable, replaceable. It has no special privilege. If someone builds a better framework tomorrow, pons users can migrate.

---

## Why pons exists

Every mainstream framework treats i18n as a plugin. The result:

| Framework | Route | French user sees |
|---|---|---|
| Rails | `/about` | `/about` (in French) |
| Phoenix | `/about` | `/about` (in French) |
| Django | `/about` | `/about` (in French) |
| Express | `/about` | `/about` (in French) |
| **pons** | `/about` · `/despre` · `/a-propos` | **`/a-propos`** (in French) |

The URL is part of the user's experience. A French user should not have to read English in the address bar to find the "about" page. **pons treats language as a routing concern, not a rendering concern.**

One handler. Three URLs. Zero duplication.

---

## Quick start

### Install

```bash
lopm add pons